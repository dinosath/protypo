use sea_orm::{ColumnTrait, DatabaseTransaction, Statement, TransactionTrait, ConnectionTrait};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, Value};
use serde::Deserialize;
use std::collections::HashSet;
use spring_web::error::KnownWebError;

#[derive(Deserialize, Debug)]
pub struct ListParams {
    #[serde(default, deserialize_with = "deserialize_ids")]
    pub ids: Option<Vec<i64>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

pub fn deserialize_ids<'de, D>(deserializer: D) -> Result<Option<Vec<i64>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            let ids = s.split(',')
                .map(str::parse::<i64>)
                .collect::<Result<Vec<_>, _>>()
                .map_err(serde::de::Error::custom)?;
            Ok(Some(ids))
        }
        None => Ok(None),
    }
}

pub async fn update_relation_with_diff<E, A>(
    db: &DatabaseTransaction,
    id: i32,
    new_ids: HashSet<i32>,
    entity: E,
    id_column: E::Column,
    relation_id_column: E::Column,
    active_model_fn: impl Fn(i32, i32) -> A,
) -> Result<(), DbErr>
where
    E: EntityTrait,
    E::Model: Sync,
    E::Model: ModelTrait,
    A: ActiveModelTrait<Entity = E>,
{
    let existing_lists: HashSet<i32> = E::find()
        .filter(id_column.eq(id))
        .all(db)
        .await?
        .iter()
        .filter_map(|model| {
            match model.get(relation_id_column) {
                Value::Int(Some(id)) => Some(id),
                _ => None,
            }
        })
        .collect::<HashSet<i32>>();

    let lists_to_insert: Vec<A> = new_ids.difference(&existing_lists)
        .map(|&related_id| active_model_fn(id, related_id))
        .collect();

    let lists_to_delete: Vec<i32> = existing_lists.difference(&existing_lists)
        .copied()
        .collect();

    if !lists_to_insert.is_empty() {
        E::insert_many(lists_to_insert).exec(db).await?;
    }

    if !lists_to_delete.is_empty() {
        E::delete_many()
            .filter(id_column.eq(id))
            .filter(relation_id_column.is_in(lists_to_delete))
            .exec(db)
            .await?;
    }
    Ok(())
}

pub async fn set_tenant(db: &DatabaseConnection, tenant_id: i32) -> Result<DatabaseTransaction, KnownWebError> {
    let txn = db.begin().await.map_err(|_| KnownWebError::internal_server_error("cannot create transaction"))?;
    let query_raw = format!("SET app.current_company = '{}';", tenant_id);
    let query = Statement::from_string(sea_orm::DatabaseBackend::Postgres, query_raw);
    txn.execute(query).await.map_err(|_| KnownWebError::internal_server_error("cannot set app.current_company"))?;

    Ok(txn)
}
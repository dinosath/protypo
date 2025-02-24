use sea_orm::{ColumnTrait, DatabaseTransaction, Statement, TransactionTrait, ConnectionTrait};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, Value};
use serde::Deserialize;
use std::collections::HashSet;

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


/// Sets the tenant for the current database connection.
///
/// This function begins a new database transaction and sets the tenant context
/// for the current session by executing a raw SQL statement. The tenant context
/// is set using the `app.current_company` variable.
///
/// # Arguments
///
/// * `db` - A reference to the `DatabaseConnection` object.
/// * `tenant_id` - The ID of the tenant to set.
///
/// # Returns
///
/// This function returns a `Result` containing a `DatabaseTransaction` object
/// if successful, or a `DbErr` if an error occurs.
///
/// # Errors
///
/// This function will return an error if the transaction cannot be started or
/// if the SQL statement execution fails.
///
/// # Examples
///
/// ```rust
/// use sea_orm::DatabaseConnection;
/// use crate::utils::set_tenant;
///
/// async fn example(db: &DatabaseConnection) {
///     let tenant_id = 1;
///     match set_tenant(db, tenant_id).await {
///         Ok(txn) => {
///             // Transaction started and tenant set successfully
///         }
///         Err(err) => {
///             // Handle error
///         }
///     }
/// }
/// ```
pub async fn begin_tenant_transaction(db: &DatabaseConnection, tenant_id: i32) -> Result<DatabaseTransaction, DbErr> {
    let txn = db.begin().await?;
    let query_raw = format!("SET app.current_company = '{}';", tenant_id);
    let query = Statement::from_string(sea_orm::DatabaseBackend::Postgres, query_raw);
    txn.execute(query).await?;

    Ok(txn)
}
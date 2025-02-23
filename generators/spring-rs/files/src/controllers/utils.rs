use sea_orm::ColumnTrait;
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
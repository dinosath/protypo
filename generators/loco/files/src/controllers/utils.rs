use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ListParams {
    #[serde(default, deserialize_with = "deserialize_ids")]
    pub ids: Option<Vec<i64>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

pub fn deserialize_ids<'de, D>(deserializer: D) -> loco_rs::Result<Option<Vec<i64>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            let ids = s
                .split(',')
                .map(str::parse::<i64>)
                .collect::<loco_rs::Result<Vec<_>, _>>()
                .map_err(serde::de::Error::custom)?;
            Ok(Some(ids))
        }
        None => Ok(None),
    }
}

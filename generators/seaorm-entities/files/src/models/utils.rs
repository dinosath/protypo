use sea_orm::prelude::*;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, ActiveModelBehavior};
use async_trait::async_trait;

#[async_trait]
pub trait Crud<E>
where
    E: EntityTrait + Sync + Send + 'static,
    <E as EntityTrait>::Model: Sync + Send,
    <E as EntityTrait>::ActiveModel: ActiveModelTrait + ActiveModelBehavior + Sync + Send,
{
    // Create or Insert a single entity
    async fn insert_one(db: &DatabaseConnection, model: <E as EntityTrait>::ActiveModel) -> Result<<E as EntityTrait>::Model, DbErr> {
        model.insert(db).await
    }

    // Insert many entities at once
    async fn insert_many(db: &DatabaseConnection, models: Vec<<E as EntityTrait>::ActiveModel>) -> Result<Vec<<E as EntityTrait>::Model>, DbErr> {
        let mut insertions = vec![];
        for model in models {
            let inserted = model.insert(db).await?;
            insertions.push(inserted);
        }
        Ok(insertions)
    }

    // Find a single entity by ID (assuming ID is i32, adjust as needed)
    async fn find_one(db: &DatabaseConnection, id: i32) -> Result<Option<<E as EntityTrait>::Model>, DbErr> {
        E::find_by_id(id).one(db).await
    }

    // Find multiple entities based on a filter
    async fn find_many<F>(db: &DatabaseConnection, filter: F) -> Result<Vec<<E as EntityTrait>::Model>, DbErr>
    where
        F: FnOnce(E::Column) -> Expr,
    {
        E::find().filter(filter(E::Column::default())).all(db).await
    }

    // Update an existing entity
    async fn update_one(db: &DatabaseConnection, model: <E as EntityTrait>::ActiveModel) -> Result<<E as EntityTrait>::Model, DbErr> {
        model.update(db).await
    }

    // Delete an entity by ID
    async fn delete_one(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        E::delete_by_id(id).exec(db).await
    }

    // Delete multiple entities based on a filter
    async fn delete_many<F>(db: &DatabaseConnection, filter: F) -> Result<DeleteResult, DbErr>
    where
        F: FnOnce(E::Column) -> Expr,
    {
        E::delete_many().filter(filter(E::Column::default())).exec(db).await
    }
}

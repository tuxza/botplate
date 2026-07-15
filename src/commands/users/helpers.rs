use crate::entities;
use sea_orm::DatabaseConnection;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use poise::serenity_prelude::{self as serenity};

pub async fn get_balance(author: &serenity::User, database: &DatabaseConnection) -> i64 {
    let user = entities::users::Entity::find()
        .filter(entities::users::Column::Id.eq(author.id.get() as i64))
        .one(database)
        .await
        .unwrap_or_default();

    match user {
        Some(token) => token.tokens.abs(),
        None => 0,
    }
}

pub async fn edit_balance(author: &serenity::User, amount: i64, database: &DatabaseConnection) {
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};

    let user_id = author.id.get() as i64;
    let user = entities::users::Entity::find_by_id(user_id)
        .one(database)
        .await
        .unwrap_or(None);

    match user {
        Some(model) => {
            let mut active_model: entities::users::ActiveModel = model.into();
            active_model.tokens = Set(active_model.tokens.unwrap() + amount);
            let _ = active_model.update(database).await;
        }
        None => {
            let new_user = entities::users::ActiveModel {
                id: Set(user_id),
                tokens: Set(amount),
                debt: Set(0),
                last_daily: Set(None),
                last_job: Set(None),
                xp: Set(0),
                level: Set(0),
            };
            let _ = new_user.insert(database).await;
        }
    }
}

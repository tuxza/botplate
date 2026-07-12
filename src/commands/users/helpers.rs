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
    !todo!("ill do it later")
    /* let user = entities::users::Entity::find()
    .filter(entities::users::Column::Id.eq(author.id.get() as i64))
    .one(database)
    .await
    .unwrap_or_default(); */
}

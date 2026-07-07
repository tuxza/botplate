use crate::entities;
use poise::serenity_prelude::{self as serenity};
use sea_orm::DatabaseConnection;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

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

#[poise::command(prefix_command, slash_command)]
pub async fn balance(
    ctx: poise::Context<'_, crate::Data, serenity::Error>,
) -> Result<(), serenity::Error> {
    let author = ctx.author();
    let balance = get_balance(author, &ctx.data().database).await;
    let _ = ctx.say(format!("Your balance is: {}", balance)).await?;
    Ok(())
}

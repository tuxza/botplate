use crate::entities;
use poise::serenity_prelude::{self as serenity};
use sea_orm::DatabaseConnection;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub async fn get_balance(author: &serenity::User, database: &DatabaseConnection) -> i64 {
    let central_bank = entities::central_bank::Entity::find()
        .filter(entities::central_bank::Column::Id.eq(1))
        .one(database)
        .await
        .unwrap_or_default();
    let balance = central_bank.map(|bank| bank.balance).unwrap_or(0).abs() as i64;
    balance
}

#[poise::command(prefix_command, slash_command)]
pub async fn balance(
    ctx: poise::Context<'_, crate::Data, serenity::Error>,
) -> Result<(), serenity::Error> {
    let author = ctx.author();
    let balance = get_balance(&author, &ctx.data().database).await;
    let _ = ctx.say(format!("Your balance is: {}", balance)).await?;
    Ok(())
}

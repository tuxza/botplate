use poise::serenity_prelude as serenity;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, crate::Data, serenity::Error>,
    _data: &crate::Data,
) -> Result<(), serenity::Error> {
    match event {
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            on_guild_join(&_data.database, new_member).await?;
        }
        _ => {}
    }
    Ok(())
}

use crate::entities;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub async fn on_guild_join(
    db: &DatabaseConnection,
    new_member: &serenity::Member,
) -> Result<(), serenity::Error> {
    let user = entities::users::ActiveModel {
        id: Set(new_member.user.id.get() as i64),
        tokens: Set(0),
        ..Default::default()
    };

    user.insert(db).await.unwrap();
    println!("User {} joined the guild.", new_member.user.name);
    Ok(())
}

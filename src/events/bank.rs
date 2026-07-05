use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ChannelId;

use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateMessage;

use crate::etc::make_numbers_pretty;

use crate::entities;
use sea_orm::DatabaseConnection;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

async fn get_money_get_bread(database: &DatabaseConnection) -> String {
    let central_bank = entities::central_bank::Entity::find()
        .filter(entities::central_bank::Column::Id.eq(1))
        .one(database)
        .await
        .unwrap_or_default();

    let balance = central_bank.map(|bank| bank.balance).unwrap_or(0);

    make_numbers_pretty(balance as u64).await
}

pub async fn send_bank_embed(
    http: &serenity::Http,
    channel_id: ChannelId,
    database: &DatabaseConnection,
) -> serenity::Result<serenity::Message> {
    let central_bank = get_money_get_bread(database).await;

    let embed = CreateEmbed::new()
        .title("central bank")
        .description(format!("tuxbux reserves: {}", central_bank))
        .color(0xFFD700)
        .field(
            format!("what made the amount in the central bank?"),
            "what a magic number.. basically, tuxzilla did a rough estimate of the value of everything in his house and now we're here ;l'",
            true,
        )
        .field(
            "what's the value of a tuxbux in the real world?",
            "one tuxbux = 0.50 usd (u.s. dollars) \n im gonna add inflation soon screw you",
            true,
        )
        .field(
            "can i trade tuxbux for real world mone-",
            "no",
            true,
        )
        .footer(
            poise::serenity_prelude::CreateEmbedFooter::new("botplate-rs | botplate reimagined | v0.1.0")
        );

    channel_id
        .send_message(http, CreateMessage::new().embed(embed))
        .await
}

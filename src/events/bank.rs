use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ChannelId;
use std::sync::Arc;

use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateMessage;

use crate::etc::fn_that_does_nothing;

/* async fn get_money_get_bread(database: &DatabaseConnection) -> u64 {
    let central_bank = entities::central_bank::Entity::find()
        .filter(entities::central_bank::Column::Id.eq(1))
        .one(database)
        .await
        .unwrap_or_default();
    central_bank.balance
} */

pub async fn bank_embed() -> CreateEmbed {
    let central_bank = fn_that_does_nothing().await;

    CreateEmbed::new()
        .title("central bank")
        .description(format!("tuxbux reserves: {}", central_bank))
        .color(0xFFD700)
        .field(
            format!("what made the amount in the central bank {}?", central_bank),
            "basically, tuxzilla did a rough estimate of the value of everything in his house and went: \n welp, this is what gives tuxbux their value. \n and that was <:true:1412454978949218487>",
            true,
        )
        .field(
            "what's the value of a tuxbux in the real world?",
            "one tuxbux = 0.50 usd (u.s. dollars)",
            true,
        )
        .field(
            "can i trade tuxbux for real world money?",
            "no",
            true,
        )
        .footer(
            poise::serenity_prelude::CreateEmbedFooter::new("botplate-rs | botplate reimagined | v0.1.0")
        )
}

pub async fn send_bank(http: &Arc<serenity::Http>) -> Result<(), serenity::Error> {
    let channel = ChannelId::new(1471369516612194314);
    let embed = bank_embed().await;
    channel
        .send_message(http, CreateMessage::new().embed(embed))
        .await?;
    Ok(())
}

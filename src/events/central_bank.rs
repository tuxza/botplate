// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later
// /src/events/central_bank.rs
#![allow(clippy::unreadable_literal)]
use crate::entities;
use crate::global::make_numbers_pretty;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ChannelId;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateMessage;
use poise::serenity_prelude::GetMessages;
use sea_orm::DatabaseConnection;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

async fn get_money_get_bread(database: &DatabaseConnection) -> String {
    let central_bank = entities::central_bank::Entity::find()
        .filter(entities::central_bank::Column::Id.eq(1))
        .one(database)
        .await
        .unwrap_or_default();
    let balance = central_bank.map_or(0, |bank| bank.balance);
    make_numbers_pretty(balance).await
}

/// Deletes every message in the channel, bulk-deleting where possible
/// (Discord only allows bulk delete on messages < 14 days old, 2-100 at a time)
/// and falling back to one-by-one deletion for anything older.
async fn purge_channel(http: &serenity::Http, channel_id: ChannelId) -> serenity::Result<()> {
    loop {
        let messages = channel_id
            .messages(http, GetMessages::new().limit(100))
            .await?;

        if messages.is_empty() {
            break;
        }

        let now = serenity::Timestamp::now();
        let two_weeks_ago = now.unix_timestamp() - (14 * 24 * 60 * 60);

        let (bulkable, old): (Vec<_>, Vec<_>) = messages
            .into_iter()
            .partition(|m| m.timestamp.unix_timestamp() > two_weeks_ago);

        if bulkable.len() >= 2 {
            let ids: Vec<_> = bulkable.iter().map(|m| m.id).collect();
            channel_id.delete_messages(http, ids).await?;
        } else {
            for m in &bulkable {
                m.delete(http).await?;
            }
        }

        for m in &old {
            m.delete(http).await?;
        }

        if bulkable.len() + old.len() < 100 {
            break;
        }
    }

    Ok(())
}

pub async fn send_bank_embed(
    http: &serenity::Http,
    channel_id: ChannelId,
    database: &DatabaseConnection,
) -> serenity::Result<serenity::Message> {
    purge_channel(http, channel_id).await?;

    let central_bank = get_money_get_bread(database).await;
    let embed = CreateEmbed::new()
        .title("central bank")
        .description(format!("tuxbux reserves: {central_bank}"))
        .color(0xFFD700)
        .field(
            "what made the amount in the central bank?".to_string(),
            "what a magic number.. basically, tuxzilla did a rough estimate of the value of everything in his house and now we're here",
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
            poise::serenity_prelude::CreateEmbedFooter::new(format!("botplate-rs | botplate reimagined | {}", env!("CARGO_PKG_VERSION"))),
        );
    channel_id
        .send_message(http, CreateMessage::new().embed(embed))
        .await
}

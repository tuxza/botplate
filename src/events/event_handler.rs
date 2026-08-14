// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// /src/events/event_handler.rs

#![allow(clippy::unreadable_literal)]

use poise::serenity_prelude as serenity;

use crate::errors::Error;
use crate::events::on_message::on_message;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, crate::Data, Error>,
    data: &crate::Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            on_guild_join(&data.database, new_member).await?;
        }
        serenity::FullEvent::ChannelDelete { channel, .. } => {
            on_channel_delete(&data.database, channel.id).await?;
        }
        serenity::FullEvent::Message { new_message } => {
            on_message(new_message, ctx, &data.xp_map, &data.database, 100).await?;
        }
        _ => {}
    }
    Ok(())
}

pub async fn on_channel_delete(
    db: &DatabaseConnection,
    channel_id: serenity::ChannelId,
) -> Result<(), Error> {
    crate::shops::db::db_delete_channel_by_cid(channel_id, db).await?;
    Ok(())
}

use crate::global;
use sea_orm::DatabaseConnection;

pub async fn on_guild_join(
    db: &DatabaseConnection,
    new_member: &serenity::Member,
) -> Result<(), Error> {
    let uid = new_member.user.id.get();
    global::ensure_user_exists(uid.cast_signed(), db).await?;

    println!("User {} joined the guild.", new_member.user.name);
    Ok(())
}

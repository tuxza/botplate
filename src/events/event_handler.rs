// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// /src/events/event_handler.rs

use poise::serenity_prelude as serenity;

use crate::errors::Error;
use crate::events::on_message::on_message;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, crate::Data, Error>,
    _data: &crate::Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            on_guild_join(&_data.database, new_member).await?;
        }
        serenity::FullEvent::ChannelDelete { channel, .. } => {
            on_channel_delete(&_data.database, channel.id).await?;
        }
        serenity::FullEvent::Message { new_message } => {
            on_message(new_message, ctx, &_data.xp_map, &_data.database, 100).await?;
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

use crate::entities;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub async fn on_guild_join(
    db: &DatabaseConnection,
    new_member: &serenity::Member,
) -> Result<(), Error> {
    let user = entities::users::ActiveModel {
        id: Set(new_member.user.id.get() as i64),
        tokens: Set(0),
        ..Default::default()
    };

    user.insert(db).await.unwrap();
    println!("User {} joined the guild.", new_member.user.name);
    Ok(())
}

// tux reminder, this works.. make a function that does something with it

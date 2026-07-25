// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

// /src/events/event_handler.rs

use poise::serenity_prelude as serenity;

use crate::errors::Error;

pub async fn event_handler(
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, crate::Data, Error>,
    _data: &crate::Data,
) -> Result<(), Error> {
    if let serenity::FullEvent::GuildMemberAddition { new_member } = event {
        on_guild_join(&_data.database, new_member).await?;
    }
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

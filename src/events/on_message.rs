// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/events/event_handler.rs

#![allow(clippy::unreadable_literal)]
use dashmap::DashMap;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Mentionable;
use rand::RngExt;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::entities::prelude::Users;
use crate::entities::types::UsersActiveModel;
use crate::errors::Error;
use crate::types::TuxBux;
use poise::serenity_prelude::UserId;

// I HATED WRITING THIS AND I HOPE IT DIES

pub async fn on_message(
    new_message: &serenity::Message,
    ctx: &serenity::Context,
    user_map: &DashMap<UserId, (i64, i64, i64)>,
    database: &DatabaseConnection,
    xp_per_level: i64,
) -> Result<(), Error> {
    if new_message.author.bot {
        return Ok(());
    }

    let uid = new_message.author.id;

    // if the user is not in the map, fetch their xp/level from the database. so yk..
    // we dont have to ask the DB everytime someone sends a message.
    if !user_map.contains_key(&uid) {
        let (xp, level) = Users::find_by_id(uid)
            .one(database)
            .await?
            .map_or((0, 0), |u| (u.xp, u.level));
        user_map.insert(uid, (xp, level, 0));
    }

    let xp_gained = rand::rng().random_range(5..=15);
    let mut leveled_up = false;
    let new_xp: i64;
    let new_level: i64;

    {
        let mut entry = user_map
            .get_mut(&uid)
            .ok_or(Error::Custom("something exploded".to_string()))?;
        entry.0 += xp_gained;

        let next_level_xp = xp_per_level * (entry.1 + 1);
        if entry.0 >= next_level_xp {
            entry.1 += 1;
            entry.0 -= next_level_xp;
            leveled_up = true;
        }

        new_xp = entry.0;
        new_level = entry.1;
    }

    if leveled_up {
        level_up(uid, new_xp, new_level, database).await?;

        let tokens_earned = rand::rng().random_range(1..=100);

        let embed = serenity::CreateEmbed::new()
            .title("level up!")
            .description(format!(
                "{} has reached level **{}**.",
                new_message.author.mention(),
                new_level
            ))
            .field("tuxbux earned", format!("**{tokens_earned} tuxbux**"), true)
            .color(0xFFD700);

        new_message
            .channel_id
            .send_message(&ctx.http, serenity::CreateMessage::new().embed(embed))
            .await?;

        crate::users::db::db_add_balance(uid, TuxBux(tokens_earned), database).await?;
    }

    Ok(())
}

// now CONTRIBUTING.md would tell you to put this in db.rs
// but i dont follow my own rules.

use crate::types::UserId64;

async fn level_up(
    uid: UserId,
    xp: i64,
    level: i64,
    database: &DatabaseConnection,
) -> Result<(), Error> {
    let active_model = UsersActiveModel {
        id: Set(UserId64::from(uid).get()),
        xp: Set(xp),
        level: Set(level),
        ..Default::default()
    };

    active_model.update(database).await?;
    Ok(())
}

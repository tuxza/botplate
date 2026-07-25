// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/channels/shops.rs

use crate::channels::helpers;
use crate::errors::Error;
use poise::serenity_prelude::{self as serenity, Mentionable};

/// manage your shops
#[poise::command(slash_command, prefix_command, subcommands("shop"))]
pub async fn create(_ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    Ok(())
}

/// create a new shop
#[poise::command(slash_command, prefix_command)]
pub async fn shop(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "The name of your new business"] channel_name: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;
    let user_id = ctx.author().id;

    let channel_id = helpers::create_new_shop(
        ctx.http(),
        guild_id,
        user_id,
        channel_name,
        &ctx.data().database,
    )
    .await?;

    ctx.say(format!("shop created! {}", channel_id.mention()))
        .await?;

    channel_id
        .send_message(
            ctx.http(),
            serenity::CreateMessage::new()
                .content(format!("welcome to your new shop! {}", user_id.mention())),
        )
        .await?;

    Ok(())
}

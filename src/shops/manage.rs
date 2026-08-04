// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/channels/shops.rs

use crate::errors::Error;
use crate::shops::helpers;
use poise::serenity_prelude::{self as serenity, Mentionable};

/// manage your shops
#[poise::command(slash_command, prefix_command, subcommands("create", "delete"))]
pub async fn shop(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    ctx.say("bro go AWAY").await?;
    Ok(())
}

use crate::shops::sell::items::remove;
use crate::shops::sell::items::sell;

#[poise::command(slash_command, prefix_command, subcommands("sell", "remove"))]
pub async fn items(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    ctx.say("no items here brother").await?;
    Ok(())
}

/// create a new shop
#[poise::command(slash_command, prefix_command)]
pub async fn create(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "The name of your new business"] channel_name: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild().ok_or(Error::Custom("no guild".to_string()))?.id; // holy shit this is so fucking funny
    let user_id = ctx.author().id;

    let channel_id = helpers::create_shop(
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

/// delete your shop
#[poise::command(slash_command, prefix_command)]
pub async fn delete(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    let user_id = ctx.author().id;
    helpers::delete_shop(ctx.http(), user_id, &ctx.data().database).await?;
    ctx.say("shop deleted").await?;
    Ok(())
}

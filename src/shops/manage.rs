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
    let uid = ctx.author().id;

    let cid = helpers::create_shop(
        ctx.http(),
        guild_id,
        uid,
        channel_name,
        &ctx.data().database,
    )
    .await?;

    ctx.say(format!("shop created! {}", cid.mention())).await?;

    cid.send_message(
        ctx.http(),
        serenity::CreateMessage::new()
            .content(format!("welcome to your new shop! {}", uid.mention())),
    )
    .await?;

    Ok(())
}

/// manage your shop
#[poise::command(slash_command, prefix_command)]
pub async fn manage(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    Ok(())
}

/// delete your shop
#[poise::command(slash_command, prefix_command)]
pub async fn delete(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    helpers::delete_shop(ctx.http(), ctx.author().id, &ctx.data().database).await?;
    ctx.say("shop deleted").await?;
    Ok(())
}

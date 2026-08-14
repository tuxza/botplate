// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// /src/etc/general.rs

#![allow(clippy::unreadable_literal)]
use poise::serenity_prelude::{self as serenity};

use crate::etc::helpers;
use crate::global;

use crate::errors::Error;

// tux reminder: this sucks make it better

/// ping the bot to check latency
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    let start = std::time::Instant::now();

    let msg = ctx.say("Pinging...!").await?;
    let edit_latency = start.elapsed().as_millis();

    let shard_manager = ctx.framework().shard_manager();
    let runners = shard_manager.runners.lock().await;

    let ws_latency_string = runners
        .get(&ctx.serenity_context().shard_id)
        .and_then(|r| r.latency).map_or_else(|| {
            "awaiting heartbeat... the bot probably just started. run a slash command and retry."
                .to_string()
        }, |d| format!("{}ms", d.as_millis()));

    msg.edit(
        ctx,
        poise::CreateReply::default().content(format!(
            "Pong! 🏓\nWebSocket Latency: **{ws_latency_string}**\nAPI Latency: **{edit_latency}ms**"
        )),
    )
    .await?;

    Ok(())
}

/// get information about botplate!
#[poise::command(slash_command)]
pub async fn info(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    // Safely destructure the Option without panicking.
    // If it fails, we return an error message to the Discord context.
    let Some(sys) = helpers::get_sysinfo() else {
        ctx.say("❌ Failed to retrieve system statistics.").await?;
        return Ok(());
    };

    let bot_uptime = ctx.data().start_time.elapsed().as_secs();

    let info_embed = serenity::CreateEmbed::new()
        .title("botplate info")
        .description("botplate is the finishing piece for a simulation of a low effort economy of the micronation of baseplate, handling everything from taxes, businesses, and jailing citizens. tux this description is so ASS make a better one")
        .field(
            "Bot Uptime",
            helpers::convert_uptime_2_human(bot_uptime),
            false,
        )
        .field(
            "Host Uptime",
            helpers::convert_uptime_2_human(sys.h_uptime),
            false,
        )
        .field(
            "OS",
            format!(
                "{} {}",
                sys.os_name.unwrap_or_else(|| "Unknown".to_string()),
                sys.os_vers.unwrap_or_else(|| "Unknown".to_string())
            ),
            false,
        )
        .field(
            "Bot Memory",
            helpers::convert_bytes_2_megabytes(sys.bot_memory),
            false,
        )
        .field(
            "Host Memory",
            format!(
                "{} / {}",
                helpers::convert_bytes_2_gigabytes(sys.h_used_memory),
                helpers::convert_bytes_2_gigabytes(sys.h_total_memory)
            ),
            false,
        )
        .footer(global::random_footer().await)
        .color(0x7289DA);

    let reply = poise::CreateReply::default().embed(info_embed);

    ctx.send(reply).await?;
    Ok(())
}

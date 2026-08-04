// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// This is the main file, if you'd like to contribute to botplate, please read the CONTRIBUTING.md file.

#![allow(clippy::unreadable_literal)]
#![allow(clippy::print_stdout)]
use poise::serenity_prelude as serenity;
use sea_orm::{Database, DatabaseConnection};
use std::time::Instant;

use dashmap::DashMap;

pub struct Data {
    pub start_time: Instant,
    pub database: DatabaseConnection,
    pub admins: u64,
    pub xp_map: DashMap<u64, (i64, i64)>,
}

mod admin;
mod entities;
mod errors;
mod etc;
mod events;
mod global;
mod shops;
mod users;

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
    let start = Instant::now();
    println!("starting botplate!");
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| errors::Error::Custom("DATABASE_URL not set in env".into()))?;

    let db = Database::connect(database_url).await?;

    let admins =
        std::env::var("ADMIN").map_err(|_| errors::Error::Custom("ADMIN not set in env".into()))?;

    let admins: i64 = admins
        .parse()
        .map_err(|_| errors::Error::Custom("ADMIN is not a valid integer".into()))?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                etc::general::ping(),
                etc::general::info(),
                users::user::balance(),
                users::user::rank(),
                users::user::daily(),
                users::user::gamble(),
                users::inventory::list::inventory(),
                shops::manage::shop(),
                shops::manage::items(),
                shops::buy::items::buy(),
                shops::list::list(),
                admin::commands::rule(),
                admin::commands::resend_rules(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("b.".into()),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::event_handler::event_handler(
                    ctx, event, framework, data,
                ))
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let target_channel = serenity::ChannelId::new(1401390175770382366);
                events::central_bank::send_bank_embed(&ctx.http, target_channel, &db).await?;
                Ok(Data {
                    start_time: start,
                    database: db,
                    admins: admins.cast_unsigned(), // we'll do something more elegant later.
                    xp_map: DashMap::new(),
                })
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN")
        .map_err(|_| errors::Error::Custom("DISCORD_TOKEN not set in env".into()))?;

    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    let elapsed_time = start.elapsed();
    println!("botplate started!");
    println!("Starting took: {} ms", elapsed_time.as_millis());

    client.start().await?;
    Ok(())
}

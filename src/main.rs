// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// This is the main file, if you'd like to contribute to botplate, please read the CONTRIBUTING.md file.

#![allow(clippy::unreadable_literal)]
#![allow(clippy::print_stdout)]
use poise::serenity_prelude::{self as serenity, UserId};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Instant;

use dashmap::DashMap;

pub struct Data {
    pub database: DatabaseConnection,
    pub user_map: DashMap<UserId, (i64, i64, i64)>,
    pub admins: Vec<u64>,
    pub start_time: Instant,
}

mod admin;
mod entities;
mod errors;
mod etc;
mod events;
mod global;
mod shops;
mod types;
mod users;

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
    let start = Instant::now();
    println!("starting botplate!");
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| errors::Error::Custom("DATABASE_URL not set in env".into()))?;

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(5)
        .min_connections(1)
        .connect_timeout(std::time::Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(opt).await?;

    let admins = std::env::var("ADMINS")
        .map_err(|_| errors::Error::Custom("ADMINS not set in env".into()))?;

    let admins: Vec<u64> = admins
        .split(',')
        .map(|s| {
            s.parse()
                .map_err(|_| errors::Error::Custom("ADMIN is not a valid integer".into()))
        })
        .collect::<Result<Vec<u64>, _>>()?;

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
                shops::buy::list::list(),
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
        // this is for when im testing, i dont have to change the channel this gets sent to
        // see i could be real smart and put it in a config.toml
        // but im just gonna let this thread explode when it cant be sent
        // and in reality, this will help when we are updating the embed as well
        // yay!
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let target_channel = serenity::ChannelId::new(1401390175770382366);

                let http = ctx.http.clone();
                let database = db.clone();

                tokio::spawn(async move {
                    if let Err(err) =
                        events::central_bank::send_bank_embed(&http, target_channel, &database)
                            .await
                    {
                        eprintln!("Failed to send central bank embed: {err}");
                    }
                });

                Ok(Data {
                    database: db,
                    user_map: DashMap::new(),
                    admins,
                    start_time: start,
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

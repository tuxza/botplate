use poise::serenity_prelude as serenity;
use sea_orm::{Database, DatabaseConnection};
use std::time::Instant;

pub struct Data {
    pub start_time: Instant,
    pub database: DatabaseConnection,
}

mod commands;
mod entities;
mod etc;
mod events;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let start = Instant::now();
    println!("starting botplate!");

    let db = Database::connect("sqlite://testing.db?mode=rw")
        .await
        .expect("failed to connect to database! screw you!");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::general::ping(),
                commands::general::info(),
                commands::users::user::balance(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("$".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let target_channel = serenity::ChannelId::new(1471369516612194314);
                events::central_bank::send_bank_embed(&ctx.http, target_channel, &db).await?;
                Ok(Data {
                    start_time: start,
                    database: db,
                })
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN").expect("HEY DUMBASS WHERES THE TOKEN");
    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    let elapsed_time = start.elapsed();
    println!("botplate started!");
    println!("Starting took: {} ms", elapsed_time.as_millis());

    client.start().await.unwrap();
}

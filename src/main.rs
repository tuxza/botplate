use poise::serenity_prelude as serenity;
use sea_orm::{Database, DatabaseConnection};
use std::time::Instant;

pub struct Data {
    pub start_time: std::time::Instant,
    pub database: DatabaseConnection,
}

mod commands;
mod etc;
mod events;

pub struct Uptime {
    pub start_time: Instant,
}

impl Uptime {
    pub fn get_uptime(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let _uptime = Uptime { start_time: start };
    println!("starting botplate!");

    let db = Database::connect("sqlite://botplate.db?mode=rwc")
        .await
        .expect("failed to connect to database! screw you!");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::general::ping(), commands::general::info()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("$".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                events::bank::send_bank(&ctx.http).await.unwrap();
                Ok(Data {
                    start_time: start,
                    database: db,
                })
            })
        })
        .build();

    dotenvy::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("HEY DUMBASS WHERES THE TOKEN");

    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    println!("botplate started!");
    let elapsed_time = start.elapsed();
    println!("Starting took: {} ms", elapsed_time.as_millis());
    client.start().await.unwrap();
}

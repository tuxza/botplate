use poise::serenity_prelude as serenity;
use sea_orm::{Database, DatabaseConnection};
use std::time::Instant;

pub struct Data {
    pub start_time: Instant,
    pub database: DatabaseConnection,
}

mod channels;
mod entities;
mod etc;
mod events;
mod global;
mod users;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    println!("starting botplate!");
    dotenvy::dotenv().ok();

    // this better?

    let database_url = std::env::var("DATABASE_URL")
        .expect("hey do you have DATABASE_URL in your env file?! you prolly should!");
    let db = Database::connect(database_url)
        .await
        .expect("failed to connect to database! screw you!");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                etc::general::ping(), // i should do that thing LLMs do where they give really obvious comments
                etc::general::info(), // info command
                users::user::balance(), // balance command
                users::user::daily(), // daily command
                users::user::gamble(), // gamble command
                channels::shops::new_shop(), // new shop command
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("$".into()),
                ..Default::default()
            },
            event_handler: |_ctx, event, framework, data| {
                Box::pin(events::event_handler::event_handler(event, framework, data))
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
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    let elapsed_time = start.elapsed();
    println!("botplate started!");
    println!("Starting took: {} ms", elapsed_time.as_millis());

    client.start().await.unwrap();
}

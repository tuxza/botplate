use poise::serenity_prelude as serenity;
use std::time::Instant;
// use std::env;

mod commands;
mod etc;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    println!("starting botplate!");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::general::ping(), commands::general::info()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("$".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(())
            })
        })
        .build();

    let token =
        ("MTUxNzE4ODI5OTMyNjc1MDg2MA.GZbKJz.1y_uZ9cAxSIFA4_5MCw1545ScIYa5bvkp-dvTw").to_string(); // this is for testing change later
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

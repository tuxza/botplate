use poise::serenity_prelude::{self as serenity};

use crate::etc;

// tux reminder: this sucks make it better

/// ping the bot to check latency
#[poise::command(prefix_command, slash_command)]
pub async fn ping(
    ctx: poise::Context<'_, crate::Data, serenity::Error>,
) -> Result<(), serenity::Error> {
    let start = std::time::Instant::now();

    let msg = ctx.say("Pinging...!").await?;
    let edit_latency = start.elapsed().as_millis();

    let shard_manager = ctx.framework().shard_manager();
    let runners = shard_manager.runners.lock().await;

    let ws_latency_string = runners
        .get(&ctx.serenity_context().shard_id)
        .and_then(|r| r.latency)
        .map(|d| format!("{}ms", d.as_millis()))
        .unwrap_or_else(|| {
            "awaiting heartbeat... the bot probably just started. run a slash command and retry."
                .to_string()
        });

    msg.edit(
        ctx,
        poise::CreateReply::default().content(format!(
            "Pong! 🏓\nWebSocket Latency: **{}**\nAPI Latency: **{}ms**",
            ws_latency_string, edit_latency
        )),
    )
    .await?;

    Ok(())
}

/// get information about botplate!
#[poise::command(slash_command)]
pub async fn info(
    ctx: poise::Context<'_, crate::Data, serenity::Error>,
) -> Result<(), serenity::Error> {
    let sys = etc::get_sysinfo().await;
    let bot_uptime = ctx.data().start_time.elapsed().as_secs();

    let info_embed = serenity::CreateEmbed::new()
        .title("botplate info")
        .description("botplate is the finishing piece for a simulation of a low effort economy of the micronation of baseplate, handling everything from taxes, businesses, and jailing citizens. tux this description is so ASS make a better one")
        .field(
            "Bot Uptime",
            etc::convert_uptime_2_human(bot_uptime).await.to_string(),
            false,
        )
        .field(
            "Host Uptime",
            etc::convert_uptime_2_human(sys.h_uptime).await.to_string(),
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
            etc::convert_bytes_2_megabytes(sys.bot_memory).await.to_string(),
            false,
        )
        .field(
            "Host Memory",
            format!(
                "{} / {}",
                etc::convert_bytes_2_gigabytes(sys.h_used_memory).await,
                etc::convert_bytes_2_gigabytes(sys.h_total_memory).await
            ),
            false,
        )
        .footer(etc::random_footer().await)
        .color(0x7289DA);

    let reply = poise::CreateReply::default().embed(info_embed);

    ctx.send(reply).await?;
    Ok(())
}

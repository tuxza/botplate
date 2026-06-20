use poise::serenity_prelude as serenity;

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: poise::Context<'_, (), serenity::Error>) -> Result<(), serenity::Error> {
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

#[poise::command(slash_command)]
pub async fn info(ctx: poise::Context<'_, (), serenity::Error>) -> Result<(), serenity::Error> {
    let info_embed = serenity::CreateEmbed::new()
        .title("testing!")
        .description("shit embed")
        // clear
        // .field("uptime", format!("{}", uptime().as_secs()), false)
        .color(0x7289DA);

    let reply = poise::CreateReply::default().embed(info_embed);

    ctx.send(reply).await?;
    Ok(())
}

/* async def info(interaction: discord.Interaction):
"""Get information about the bot"""
bot_uptime = datetime.now() - START_TIME
hours, remainder = divmod(int(bot_uptime.total_seconds()), 3600)
minutes, seconds = divmod(remainder, 60)
days, hours = divmod(hours, 24)

host_uptime_seconds = int(psutil.boot_time())
host_uptime = datetime.now() - datetime.fromtimestamp(host_uptime_seconds)
host_hours, host_remainder = divmod(int(host_uptime.total_seconds()), 3600)
host_minutes, host_seconds = divmod(host_remainder, 60)
host_days, host_hours = divmod(host_hours, 24)

os_info = get_os_info()

embed = discord.Embed(
    title="some stuff about botplate:", color=discord.Color.blue()
)
embed.add_field(
    name="what is botplate?",
    value="botplate is the finishing piece for a simulation of a low effort capitalist society for the town of baseplate, handling everything from taxes, setting up a business, and jailing citizens. pretty much capitalism without the greed of the industrial revolution, and whatever dementija tells you about.",
    inline=True,
)
embed.add_field(
    name="bot version", value="warty warthog stable v2.00", inline=False
)
embed.add_field(
    name="network latency", value=f"{round(bot.latency * 1000)}ms", inline=False
)
embed.add_field(
    name="bot uptime", value=f"{days}d {hours}h {minutes}m {seconds}s", inline=False
)
embed.add_field(
    name="host uptime",
    value=f"{host_days}d {host_hours}h {host_minutes}m {host_seconds}s",
    inline=False,
)
embed.add_field(name="host operating system", value=os_info, inline=False)
embed.set_footer(text="this is actually modified code from bloxbot")

await interaction.response.send_message(embed=embed) */

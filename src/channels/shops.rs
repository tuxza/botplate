use crate::channels::helpers;
use poise::serenity_prelude::{self as serenity, ChannelId, Mentionable};

/// create a new shop
#[poise::command(slash_command)]
pub async fn new_shop(
    ctx: poise::Context<'_, crate::Data, serenity::Error>,
    #[description = "The name of your new business"] channel_name: String,
) -> Result<(), serenity::Error> {
    let guild_id = ctx.guild().unwrap().id;
    let user_id = ctx.author().id;

    let channel_id = helpers::create_new_shop(&ctx.http(), guild_id, user_id, channel_name).await?;

    ctx.say(format!("shop created! {}", channel_id.mention()))
        .await?;
    Ok(())
}

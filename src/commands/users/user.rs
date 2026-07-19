use poise::serenity_prelude::{self as serenity};

use crate::commands::users::helpers;

#[poise::command(prefix_command, slash_command)]
pub async fn balance(
    ctx: poise::Context<'_, crate::Data, serenity::Error>,
) -> Result<(), serenity::Error> {
    let author = ctx.author();
    let balance = helpers::get_balance(author, &ctx.data().database).await;
    let _ = ctx.say(format!("Your balance is: {}", balance)).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn daily(
    ctx: poise::Context<'_, crate::Data, serenity::Error>,
) -> Result<(), serenity::Error> {
    let author = ctx.author();
    let last = helpers::last_daily(author, &ctx.data().database).await;

    if !helpers::can_claim_daily(last) {
        ctx.say("you already claimed your daily today! come back later please.")
            .await?;
        return Ok(());
    }

    helpers::edit_balance(author, 100, &ctx.data().database).await;
    helpers::set_last_daily(author, chrono::Utc::now().timestamp(), &ctx.data().database).await;

    ctx.say("Claimed 100 tokens!").await?;
    Ok(())
}

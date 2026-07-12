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

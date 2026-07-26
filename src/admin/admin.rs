use crate::admin::helpers::rulez;
use crate::errors::Error;

#[poise::command(prefix_command)]
pub async fn rule(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "rule number"] number: u8,
) -> Result<(), Error> {
    let text = rulez(number).await?;
    ctx.say(text).await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn resend_rules(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    for i in 1..=10 {
        let text = rulez(i).await?;
        ctx.say(text).await?;
    }
    Ok(())
}

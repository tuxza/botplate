use crate::admin::helpers::rulez;
use crate::errors::Error;

// this doesnt have an admin check because
// anyone can review the rules! yay!
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
    let admins = ctx.data().admins;
    let author = ctx.author();
    if !crate::global::is_admin(author.id.get() as i64, admins).await {
        return Err(Error::Custom("you are not an admin".to_string()));
    }

    for i in 1..=15 {
        let text = rulez(i).await?;
        ctx.say(text).await?;
    }
    Ok(())
}

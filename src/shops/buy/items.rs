use crate::errors::Error;

#[poise::command(slash_command, prefix_command)]
pub async fn buy(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "item name"] name: String,
    #[description = "quantity"] quantity: i64,
) -> Result<(), Error> {
    let channel_id = ctx.channel_id();

    Ok(())
}

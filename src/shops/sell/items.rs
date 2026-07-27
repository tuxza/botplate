use crate::shops::sell::helpers::sell_item;

use crate::errors::Error;

#[poise::command(slash_command, prefix_command, subcommands("sell", "delete"))]
pub async fn items(_ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn sell(
    _ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "name of item to sell"] name: String,
    #[description = "description of item to sell"] description: String,
    #[description = "price per unit"] price: i64,
    #[description = "quantity to sell"] quantity: i64,
) -> Result<(), Error> {
    let cid = i64::from(_ctx.channel_id());
    let item_type = "default".to_string();
    let database = &_ctx.data().database;

    sell_item(cid, name, description, item_type, price, quantity, database).await?;

    _ctx.say("item listed").await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn delete(_ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    Ok(())
}

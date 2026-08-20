use crate::errors::Error;
use crate::shops::buy::db::db_list_items;
use poise::serenity_prelude::CreateEmbed;

#[poise::command(prefix_command, slash_command)]
pub async fn list(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    let cid = ctx.channel_id();

    let items = db_list_items(cid.get().cast_signed(), &ctx.data().database).await?;

    let mut embed = CreateEmbed::new()
        .title("listed items")
        .description("here's what this shop is selling:");

    for item in items {
        let price = item.price;

        embed = embed.field(
            &item.name,
            format!("{}\n**T$:** {} x{}", item.description, price, item.quantity),
            false,
        );
    }

    let reply = poise::CreateReply::default().embed(embed);

    ctx.send(reply).await?;

    Ok(())
}

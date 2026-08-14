use crate::{errors::Error, global::random_footer};
use poise::serenity_prelude::CreateEmbed;

/// check your inventory
#[poise::command(prefix_command, slash_command)]
pub async fn inventory(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    let author = ctx.author();
    let items =
        crate::users::inventory::db::db_get_inventory(author.id.get(), &ctx.data().database)
            .await?;

    if items.is_empty() {
        ctx.say("your inventory is empty!").await?;
        return Ok(());
    }

    let list = items
        .iter()
        .map(|(name, qty)| format!("**{name}** x{qty}"))
        .collect::<Vec<_>>()
        .join("\n");

    let embed = CreateEmbed::new()
        .color(0x7492B9)
        .title("your inventory")
        .field("items", format!("\n{list}"), false)
        .footer(random_footer().await);

    let reply = poise::CreateReply::default().embed(embed);

    ctx.send(reply).await?;

    Ok(())
}

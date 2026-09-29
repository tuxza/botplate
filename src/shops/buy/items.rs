// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later//

// /src/shops/buy/items.rs

use crate::shops::buy::db::db_buy_item;
use crate::shops::db::db_verify_shop_exists;
use crate::types::MAX_QUANTITY;

use crate::{errors::Error, types::Quantity};

/// Buy an item from a shop.
#[poise::command(slash_command, prefix_command)]
pub async fn buy(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "name of the item to buy"] item: String,
    #[description = "how many you want"] quantity: i64,
) -> Result<(), Error> {
    let uid = ctx.author().id;
    let cid = ctx.channel_id();

    if !db_verify_shop_exists(cid, &ctx.data().database).await? {
        ctx.say("this isn't a shop!").await?;
        return Ok(());
    }

    if quantity <= 0 {
        ctx.say("quantity gotta be positive.. bum.").await?;
        return Ok(());
    }

    if quantity > MAX_QUANTITY {
        ctx.say(format!(
            "You can't buy more than {} of `{}`.",
            MAX_QUANTITY, item
        ))
        .await?;
        return Ok(());
    }

    let purchased = db_buy_item(
        uid,
        cid,
        item.clone(),
        Quantity(quantity),
        &ctx.data().database,
    )
    .await?;

    if !purchased {
        ctx.say(format!(
            "Could not buy `{}`. Check that it exists, there is enough stock, and you have enough cash.",
            item
        ))
        .await?;
        return Ok(());
    }

    ctx.say(format!("Successfully bought {}x {}!", quantity, item))
        .await?;

    Ok(())
}

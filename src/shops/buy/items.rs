// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later//

// /src/shops/buy/items.rs

use crate::shops::buy::db::db_remove_item;
use crate::shops::db::db_get_shop_owner_id;
use crate::shops::db::db_verify_shop;
use crate::users::inventory::db::db_add_inv_item;

use crate::{errors::Error, shops::buy::db::db_get_item};

#[poise::command(slash_command, prefix_command)]
pub async fn buy(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "name of the item to buy"] item: String,
    #[description = "how many you want"] quantity: i64,
) -> Result<(), Error> {
    let uid = ctx.author().id.get();
    let cid = ctx.channel_id().get();

    if !db_verify_shop(uid, cid, &ctx.data().database).await? {
        ctx.say("this isn't a shop!").await?;
        return Ok(());
    }

    let db_item = db_get_item(&item, &ctx.data().database).await?;

    let Some(found_item) = db_item else {
        ctx.say(format!("No item found matching `{item}`.")).await?;
        return Ok(());
    };

    let item_id = found_item.id;
    let acquired_price = found_item.price * quantity;

    let amount = -acquired_price;
    let database = &ctx.data().database;

    crate::users::helpers::edit_balance(uid, amount, database).await?;

    db_add_inv_item(uid, item_id, quantity, acquired_price, &ctx.data().database).await?;

    let owner_id = db_get_shop_owner_id(cid, &ctx.data().database).await?;

    let Some(owner) = owner_id else {
        return Err(Error::Custom(
            "Internal error. Could not find shop owner.".to_string(),
        ));
    }; // this check might not be neccessary, but nonetheless
    // shadow user_id and amount for the shop owner now!!
    let uid = owner.get();
    let amount = acquired_price;

    crate::users::helpers::edit_balance(uid, amount, database).await?;

    // couldnt think of a name, sorry
    // maybe ill make this standard across the codebase...
    // if i do this more than once LOL
    let perchance = db_remove_item(cid, &found_item.name, quantity, &ctx.data().database).await?;

    if !perchance {
        ctx.say(format!("Not enough stock of `{}`.", found_item.name))
            .await?;
        return Ok(());
    }

    ctx.say(format!(
        "Successfully bought {}x {}!",
        quantity, found_item.name
    ))
    .await?;

    Ok(())
}

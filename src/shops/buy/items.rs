// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later//

// /src/shops/buy/items.rs

use crate::shops::buy::db::db_remove_item;
use crate::shops::db::db_verify_shop_exists;
use crate::users::inventory::db::db_add_inv_item;

use poise::serenity_prelude::UserId;

use crate::{
    errors::Error, shops::buy::db::db_get_item, shops::db::db_get_shop_owner_id, types::TuxBux,
};

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

    let db_item = db_get_item(cid, &item, &ctx.data().database).await?;

    let Some(found_item) = db_item else {
        ctx.say(format!("No item found matching `{item}`.")).await?;
        return Ok(());
    };

    let item_id = found_item.id;
    let item_quantity = found_item.quantity;
    let acquired_price = found_item.price * quantity;

    if item_quantity < quantity {
        ctx.say(format!("Not enough stock of `{}`.", found_item.name))
            .await?;
        return Ok(());
    }

    let amount = -acquired_price;
    let database = &ctx.data().database;

    crate::users::db::db_deduct_balance(uid, TuxBux(amount), database).await?;

    db_add_inv_item(uid, item_id, quantity, acquired_price, &ctx.data().database).await?;

    let owner = db_get_shop_owner_id(cid, &ctx.data().database).await?;
    let Some(owner) = owner else {
        return Err(Error::Custom(
            "Internal error. Could not find shop owner.".to_string(),
        ));
    };

    let uid = UserId::from(owner.cast_unsigned());
    let amount = acquired_price;

    crate::users::db::db_add_balance(uid, amount.into(), database).await?; // database -> signed -> unsigned -> signed -> database
    db_remove_item(cid, &found_item.name, quantity, &ctx.data().database).await?;

    ctx.say(format!(
        "Successfully bought {}x {}!",
        quantity, found_item.name
    ))
    .await?;

    Ok(())
}

// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// /src/shops/sell/items.rs

use crate::errors::Error;
use crate::shops::buy::db::db_remove_item;
use crate::shops::db::db_verify_shop_owner;
use crate::shops::sell::db;
use crate::types::TuxBux;

/// List an item for sale in your shop.
#[poise::command(slash_command, prefix_command)]
pub async fn sell(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "item name"] name: String,
    #[description = "price in tuxbux"] price: i64,
    #[description = "how many to list"] quantity: i64,
    #[description = "description"] description: Option<String>,
) -> Result<(), Error> {
    let uid = ctx.author().id;
    let cid = ctx.channel_id();

    if !db_verify_shop_owner(uid, cid, &ctx.data().database).await? {
        ctx.say("this isn't your shop!").await?;
        return Ok(());
    }

    if price <= 0 || quantity <= 0 {
        ctx.say("price and quantity gotta be positive.. bum.")
            .await?;
        return Ok(());
    }

    let msg = format!("listed **{name}** x{quantity} for {price} tuxbux each");

    db::add_item(
        cid,
        name,
        description.unwrap_or_default(),
        "product".to_string(),
        TuxBux(price),
        quantity,
        &ctx.data().database,
    )
    .await?;

    ctx.say(msg).await?;
    Ok(())
}

/// remove an item from your shop
#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "item name"] name: String,
    #[description = "quantity"] quantity: i64,
) -> Result<(), Error> {
    let uid = ctx.author().id;
    let cid = ctx.channel_id();

    if !db_verify_shop_owner(uid, cid, &ctx.data().database).await? {
        ctx.say("this isn't your shop!").await?;
        return Ok(());
    }

    db_remove_item(cid, name.as_str(), quantity, &ctx.data().database).await?;
    ctx.say(format!("removed {name} x{quantity}")).await?;
    Ok(())
}

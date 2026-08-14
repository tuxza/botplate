// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// /src/shops/sell/items.rs

use crate::errors::Error;
use crate::shops::buy::db::db_remove_item;
use crate::shops::db::db_verify_shop_owner;
use crate::shops::sell::db;

#[poise::command(slash_command, prefix_command)]
pub async fn sell(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "item name"] name: String,
    #[description = "price in tuxbux"] price: i64,
    #[description = "how many to list"] quantity: i64,
    #[description = "description"] description: Option<String>,
) -> Result<(), Error> {
    let uid = ctx.author().id.get() as i64;
    let cid = ctx.channel_id().get() as i64;

    if !db_verify_shop_owner(uid, cid, &ctx.data().database).await? {
        ctx.say("this isn't your shop!").await?;
        return Ok(());
    }

    if price <= 0 || quantity <= 0 {
        ctx.say("price and quantity gotta be positive.. bum.")
            .await?;
        return Ok(());
    }

    db::add_item(
        cid,
        name.clone(),
        description.unwrap_or_default(),
        "product".to_string(),
        price,
        quantity,
        &ctx.data().database,
    )
    .await?;

    ctx.say(format!(
        "listed **{}** x{} for {} tuxbux each",
        name, quantity, price
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "item name"] name: String,
    #[description = "quantity"] quantity: i64,
) -> Result<(), Error> {
    let uid = ctx.author().id.get() as i64;
    let cid = ctx.channel_id().get() as i64;

    if !db_verify_shop_owner(uid, cid, &ctx.data().database).await? {
        ctx.say("this isn't your shop!").await?;
        return Ok(());
    }

    db_remove_item(cid, name.as_str(), quantity, &ctx.data().database).await?;
    ctx.say(format!("removed {} x{}", name, quantity)).await?;
    Ok(())
}

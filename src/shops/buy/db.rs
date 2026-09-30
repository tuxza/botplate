// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// /src/shops/buy/db.rs

use crate::entities::items::Model;

use crate::entities::prelude::Items;
use crate::entities::types::ItemsColumn;
use crate::types::{ChannelId64, Quantity};
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use poise::serenity_prelude::ChannelId;

use crate::entities::prelude::{Inventory, Users};
use crate::entities::types::{InventoryActiveModel, InventoryColumn, UsersColumn};
use crate::types::UserId64;
use poise::serenity_prelude::UserId;
use sea_orm::TransactionError;
use sea_orm::{ActiveValue::Set, TransactionTrait};

// note to anyone reading this code, this was written by codex using GPT-5.6 Luna
// I'm tired. don't judge me.
// funny enough, besides all the boilerplate in main, this is the largest
// AI written piece in the codebase LOL

/// Atomically buys an item from a shop.
///
/// The entire purchase happens inside one database transaction:
///
/// 1. Verify the item exists and has enough stock.
/// 2. Find the shop owner.
/// 3. Deduct the buyer's balance.
/// 4. Add the item to the buyer's inventory.
/// 5. Credit the shop owner.
/// 6. Remove the purchased quantity from the shop.
///
/// If any step fails, the entire transaction is rolled back.
pub async fn db_buy_item(
    uid: UserId,
    cid: ChannelId,
    name: String,
    quantity: Quantity,
    database: &DatabaseConnection,
) -> Result<bool, DbErr> {
    let quantity = quantity.get();

    if quantity <= 0 {
        return Ok(false);
    }

    database
        .transaction(|txn| {
            Box::pin(async move {
                // find item
                let Some(item) = Items::find()
                    .filter(ItemsColumn::Name.eq(name))
                    .filter(ItemsColumn::OriginCid.eq(ChannelId64::from(cid)))
                    .one(txn)
                    .await?
                else {
                    return Ok(false);
                };

                // Make the stock check part of the transaction.
                if item.quantity < quantity {
                    return Ok(false);
                }

                let total_price = item
                    .price
                    .checked_mul(quantity)
                    .ok_or_else(|| DbErr::Custom("purchase price overflow".to_string()))?;

                // Find the shop owner.
                let Some(shop) = crate::entities::prelude::Channels::find()
                    .filter(crate::entities::types::ChannelsColumn::Cid.eq(ChannelId64::from(cid)))
                    .one(txn)
                    .await?
                else {
                    return Ok(false);
                };

                let owner = UserId::new(shop.uid.cast_unsigned());

                // Deduct the buyer's balance.
                let buyer_update = Users::update_many()
                    .col_expr(
                        UsersColumn::Tokens,
                        Expr::col(UsersColumn::Tokens).sub(total_price),
                    )
                    .filter(UsersColumn::Id.eq(UserId64::from(uid)))
                    .filter(UsersColumn::Tokens.gte(total_price))
                    .exec(txn)
                    .await?;

                if buyer_update.rows_affected != 1 {
                    return Ok(false);
                }

                // Add the item to the buyer's inventory.
                Inventory::insert(InventoryActiveModel {
                    uid: Set(UserId64::from(uid).get()),
                    item_id: Set(item.id),
                    quantity: Set(quantity),
                    acquired_price: Set(total_price),
                    can_resell: Set(true),
                })
                .on_conflict(
                    OnConflict::columns([InventoryColumn::Uid, InventoryColumn::ItemId])
                        .value(
                            InventoryColumn::Quantity,
                            Expr::cust("inventory.quantity + excluded.quantity"),
                        )
                        .to_owned(),
                )
                .exec(txn)
                .await?;

                // Credit the shop owner.
                //
                // The buyer and owner may be the same person. That's fine:
                // the transaction still produces the correct net balance.
                Users::update_many()
                    .col_expr(
                        UsersColumn::Tokens,
                        Expr::col(UsersColumn::Tokens).add(total_price),
                    )
                    .filter(UsersColumn::Id.eq(UserId64::from(owner)))
                    .exec(txn)
                    .await?;

                // Remove the stock.
                //
                // IMPORTANT: the quantity check is repeated here so this
                // update itself is authoritative if concurrent purchases
                // are ever possible.
                let stock_update = Items::update_many()
                    .col_expr(
                        ItemsColumn::Quantity,
                        Expr::col(ItemsColumn::Quantity).sub(quantity),
                    )
                    .filter(ItemsColumn::Id.eq(item.id))
                    .filter(ItemsColumn::Quantity.gte(quantity))
                    .exec(txn)
                    .await?;

                if stock_update.rows_affected != 1 {
                    return Ok(false);
                }

                Ok(true)
            })
        })
        .await
        .map_err(|e| match e {
            TransactionError::Connection(err) => err,
            TransactionError::Transaction(err) => err,
        })
}

/// List all the items available in a shop.
///
/// Returns a vector of [`Model`] items.
pub async fn db_list_items(
    cid: ChannelId,
    database: &DatabaseConnection,
) -> Result<Vec<Model>, DbErr> {
    let items = Items::find()
        .filter(ItemsColumn::OriginCid.eq(ChannelId64::from(cid)))
        .all(database)
        .await?;
    Ok(items)
}

/// Get an item by its name.
///
/// Returns an [`Option`] of [`Model`] if the item is found.
pub async fn _db_get_item(
    cid: ChannelId,
    item: &str,
    database: &DatabaseConnection,
) -> Result<Option<Model>, DbErr> {
    let item = Items::find()
        .filter(ItemsColumn::Name.eq(item))
        .filter(ItemsColumn::OriginCid.eq(ChannelId64::from(cid)))
        .one(database)
        .await?;
    Ok(item)
}

/// Remove an item from the shop.
///
/// Returns `true` if the item was removed, `false` otherwise.
pub async fn db_remove_item(
    cid: ChannelId,
    name: &str,
    quantity: Quantity,
    database: &DatabaseConnection,
) -> Result<bool, DbErr> {
    let result = Items::update_many()
        .col_expr(
            ItemsColumn::Quantity,
            Expr::col(ItemsColumn::Quantity).sub(quantity),
        )
        .filter(ItemsColumn::Name.eq(name))
        .filter(ItemsColumn::OriginCid.eq(ChannelId64::from(cid)))
        .filter(ItemsColumn::Quantity.gte(quantity))
        .exec(database)
        .await?;

    Ok(result.rows_affected > 0)
}

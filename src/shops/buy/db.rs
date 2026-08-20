// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// /src/shops/buy/db.rs

use crate::entities::items::Model;

use crate::entities::prelude::Items;
use crate::entities::types::ItemsColumn;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

/// List all the items available in a shop.
///
/// Returns a vector of [`Model`] items.
pub async fn db_list_items(cid: i64, database: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    let items = Items::find()
        .filter(ItemsColumn::OriginCid.eq(cid))
        .all(database)
        .await?;
    Ok(items)
}

/// Get an item by its name.
///
/// Returns an [`Option`] of [`Model`] if the item is found.
pub async fn db_get_item(
    item: &str,
    database: &DatabaseConnection,
) -> Result<Option<Model>, DbErr> {
    let item = Items::find()
        .filter(ItemsColumn::Name.eq(item))
        .one(database)
        .await?;
    Ok(item)
}

/// Get an item by its ID.
///
/// Returns an [`Option`] of [`Model`] if the item is found.
pub async fn _db_get_item_by_id(
    item_id: i64,
    database: &DatabaseConnection,
) -> Result<Option<Model>, DbErr> {
    let item = Items::find()
        .filter(ItemsColumn::Id.eq(item_id))
        .one(database)
        .await?;
    Ok(item)
}

/// Remove an item from the shop.
///
/// Returns `true` if the item was removed, `false` otherwise.
pub async fn db_remove_item(
    cid: i64,
    name: &str,
    quantity: i64,
    database: &DatabaseConnection,
) -> Result<bool, DbErr> {
    let result = Items::update_many()
        .col_expr(
            ItemsColumn::Quantity,
            Expr::col(ItemsColumn::Quantity).sub(quantity),
        )
        .filter(ItemsColumn::Name.eq(name))
        .filter(ItemsColumn::OriginCid.eq(cid))
        .filter(ItemsColumn::Quantity.gte(quantity))
        .exec(database)
        .await?;

    Ok(result.rows_affected > 0)
}

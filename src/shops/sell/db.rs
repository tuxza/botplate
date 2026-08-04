// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// /src/shops/sell/db.rs

use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::entities::prelude::Items;
use crate::entities::types::{ItemsActiveModel, ItemsColumn};

pub async fn add_item(
    cid: u64,
    name: String,
    description: String,
    item_type: String,
    price: i64,
    quantity: i64,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    let existing = Items::find()
        .filter(ItemsColumn::Name.eq(name.clone()))
        .filter(ItemsColumn::OriginCid.eq(cid))
        .one(database)
        .await?;

    if let Some(model) = existing {
        let mut active_model: ItemsActiveModel = model.clone().into();
        active_model.quantity = Set(model.quantity + quantity);
        active_model.price = Set(price);
        active_model.description = Set(description);
        active_model.item_type = Set(item_type);
        Items::update(active_model).exec(database).await?;
    } else {
        let active_model = ItemsActiveModel {
            name: Set(name),
            description: Set(description),
            item_type: Set(item_type),
            price: Set(price),
            quantity: Set(quantity),
            origin_cid: Set(Some(cid)),
            ..Default::default()
        };
        Items::insert(active_model).exec(database).await?;
    }

    Ok(())
}

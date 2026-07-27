use crate::entities::prelude::Channels;
use crate::entities::types::ChannelsColumn;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::entities::prelude::Items;
use crate::entities::types::*;

pub async fn add_item(
    cid: i64,
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

    match existing {
        Some(model) => {
            let mut active_model: ItemsActiveModel = model.clone().into();
            active_model.quantity = Set(model.quantity + quantity);
            active_model.price = Set(price);
            active_model.description = Set(description);
            active_model.item_type = Set(item_type);
            Items::update(active_model).exec(database).await?;
        }
        None => {
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
    }

    Ok(())
}

pub async fn remove_item(
    cid: i64,
    name: String,
    quantity: i64,
    database: &DatabaseConnection,
) -> Result<bool, DbErr> {
    let existing = Items::find()
        .filter(ItemsColumn::Name.eq(name))
        .filter(ItemsColumn::OriginCid.eq(cid))
        .one(database)
        .await?;

    let Some(model) = existing else {
        return Ok(false);
    };

    let remaining = (model.quantity - quantity).max(0);

    let mut active_model: ItemsActiveModel = model.into();
    active_model.quantity = Set(remaining);
    Items::update(active_model).exec(database).await?;

    Ok(true)
}

pub async fn verify_shop(uid: i64, cid: i64, database: &DatabaseConnection) -> Result<bool, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Cid.eq(cid))
        .one(database)
        .await?;

    Ok(match channel {
        Some(c) => c.uid == uid,
        None => false,
    })
}

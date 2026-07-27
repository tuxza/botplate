use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};

use crate::entities::prelude::Items;
use crate::entities::types::*;

pub async fn sell_item(
    cid: i64,
    name: String,
    description: String,
    item_type: String,
    price: i64,
    quantity: i64,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    let active_model = ItemsActiveModel {
        name: Set(name),
        description: Set(description),
        item_type: Set(item_type),
        price: Set(price),
        quantity: Set(quantity),
        origin_cid: Set(Some(cid)),
        ..Default::default()
    };

    Items::insert(active_model)
        .on_conflict(
            OnConflict::columns([ItemsColumn::Name, ItemsColumn::OriginCid])
                .update_column(ItemsColumn::Quantity) // careful: this REPLACES, doesn't add
                .to_owned(),
        )
        .exec(database)
        .await?;

    Ok(())
}

// TUX REMINDER : MAKE SURE TO ADD A CHECK THAT ONLY THE USER THAT OWNS THE SHOP CAN SELL ITEMS
// ANOTHER REMINDER : MAKE SURE TO VERIFY THAT A CHANNEL IS A SHOP
// probably can do that all in one function to save DB calls

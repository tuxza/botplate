use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};

use crate::entities::prelude::Inventory;
use crate::entities::types::InventoryActiveModel;

pub async fn db_add_inv_item(
    uid: i64,
    item_id: i32,
    quantity: i64,
    acquired_price: i64,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    let existing = Inventory::find_by_id((uid.into(), item_id.into()))
        .one(database)
        .await?;

    match existing {
        Some(model) => {
            let mut active_model: InventoryActiveModel = model.clone().into();
            active_model.quantity = Set(model.quantity + quantity);
            Inventory::update(active_model).exec(database).await?;
        }
        None => {
            let active_model = InventoryActiveModel {
                uid: Set(uid),
                item_id: Set(item_id.into()),
                quantity: Set(quantity),
                acquired_price: Set(acquired_price),
                can_resell: Set(true),
            };
            Inventory::insert(active_model).exec(database).await?;
        }
    }

    Ok(())
}

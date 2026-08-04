use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};

use crate::entities::prelude::Inventory;
use crate::entities::types::InventoryActiveModel;

// FUCK THE GITHUB ISSUE I JUST REALIZED ITS THE 29TH

pub async fn db_add_inv_item(
    uid: u64,
    item_id: i64,
    quantity: i64,
    acquired_price: i64,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    let existing = Inventory::find_by_id((uid, item_id)).one(database).await?;

    if let Some(model) = existing {
        let mut active_model: InventoryActiveModel = model.clone().into();
        active_model.quantity = Set(model.quantity + quantity);
        Inventory::update(active_model).exec(database).await?;
    } else {
        let active_model = InventoryActiveModel {
            uid: Set(uid),
            item_id: Set(item_id),
            quantity: Set(quantity),
            acquired_price: Set(acquired_price),
            can_resell: Set(true),
        };
        Inventory::insert(active_model).exec(database).await?;
    }

    Ok(())
}

pub async fn db_get_inventory(
    uid: u64,
    database: &DatabaseConnection,
) -> Result<Vec<(String, i64)>, DbErr> {
    use crate::entities::prelude::{Inventory, Items};
    use crate::entities::types::InventoryColumn;
    use sea_orm::{ColumnTrait, QueryFilter};

    let entries = Inventory::find()
        .filter(InventoryColumn::Uid.eq(uid))
        .find_also_related(Items)
        .all(database)
        .await?;

    Ok(entries
        .into_iter()
        .filter_map(|(inv, item)| item.map(|i| (i.name, inv.quantity)))
        .collect())
}

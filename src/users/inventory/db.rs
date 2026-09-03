use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};

use crate::entities::prelude::Inventory;
use crate::entities::types::InventoryActiveModel;
use crate::types::{Quantity, UserId64};
use poise::serenity_prelude::UserId;

pub async fn db_add_inv_item(
    uid: UserId,
    item_id: i64,
    quantity: Quantity,
    acquired_price: i64,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    let existing = Inventory::find_by_id((UserId64::from(uid).get(), item_id))
        .one(database)
        .await?;

    if let Some(model) = existing {
        let mut active_model: InventoryActiveModel = model.clone().into();
        active_model.quantity = Set(model.quantity + quantity.0);
        Inventory::update(active_model).exec(database).await?;
    } else {
        let active_model = InventoryActiveModel {
            uid: Set(UserId64::from(uid).get()),
            item_id: Set(item_id),
            quantity: Set(Quantity::from(quantity).into()),
            acquired_price: Set(acquired_price),
            can_resell: Set(true),
        };
        Inventory::insert(active_model).exec(database).await?;
    }

    Ok(())
}

pub async fn db_get_inventory(
    uid: UserId,
    database: &DatabaseConnection,
) -> Result<Vec<(String, i64)>, DbErr> {
    use crate::entities::prelude::{Inventory, Items};
    use crate::entities::types::InventoryColumn;
    use sea_orm::{ColumnTrait, QueryFilter};

    let entries = Inventory::find()
        .filter(InventoryColumn::Uid.eq(UserId64::from(uid).get()))
        .find_also_related(Items)
        .all(database)
        .await?;

    Ok(entries
        .into_iter()
        .filter_map(|(inv, item)| item.map(|i| (i.name, inv.quantity)))
        .collect())
}

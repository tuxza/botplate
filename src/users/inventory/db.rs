use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait, sea_query};

use crate::entities::prelude::Inventory;
use crate::entities::types::InventoryActiveModel;
use crate::entities::types::InventoryColumn;
use crate::types::{Quantity, UserId64};
use poise::serenity_prelude::UserId;

pub async fn _db_add_inv_item(
    uid: UserId,
    item_id: i64,
    quantity: Quantity,
    acquired_price: i64,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    let active_model = InventoryActiveModel {
        uid: Set(UserId64::from(uid).get()),
        item_id: Set(item_id),
        quantity: Set(quantity.into()),
        acquired_price: Set(acquired_price),
        can_resell: Set(true),
    };

    Inventory::insert(active_model)
        .on_conflict(
            sea_query::OnConflict::columns([InventoryColumn::Uid, InventoryColumn::ItemId])
                .value(
                    InventoryColumn::Quantity,
                    sea_query::Expr::cust("inventory.quantity + excluded.quantity"),
                )
                .to_owned(),
        )
        .exec(database)
        .await?;

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

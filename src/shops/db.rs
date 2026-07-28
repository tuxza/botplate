// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/channels/db.rs
use crate::entities::prelude::Channels;
use crate::entities::prelude::Items;
use crate::entities::types::{ChannelsActiveModel, ChannelsColumn, ItemsActiveModel, ItemsColumn};
use poise::serenity_prelude::{ChannelId, UserId};
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

pub async fn db_create_channel(
    new_channel_id: ChannelId,
    user_id: UserId,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    crate::global::ensure_user_exists(user_id.get() as i64, database).await?;

    let active_model = ChannelsActiveModel {
        cid: Set(new_channel_id.get() as i64),
        uid: Set(user_id.get() as i64),
        in_stock_market: Set(false),
    };

    Channels::insert(active_model)
        .on_conflict(
            OnConflict::column(ChannelsColumn::Cid)
                .update_column(ChannelsColumn::Uid)
                .to_owned(),
        )
        .exec(database)
        .await?;

    Ok(())
}

pub async fn db_get_shop_channel_id(
    user_id: UserId,
    database: &DatabaseConnection,
) -> Result<Option<ChannelId>, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Uid.eq(user_id.get() as i64))
        .one(database)
        .await?;
    Ok(channel.map(|c| ChannelId::new(c.cid as u64)))
}

pub async fn db_delete_shop(user_id: UserId, database: &DatabaseConnection) -> Result<(), DbErr> {
    Channels::delete_many()
        .filter(ChannelsColumn::Uid.eq(user_id.get() as i64))
        .exec(database)
        .await?;
    Ok(())
}

pub async fn db_user_has_shop(
    user_id: UserId,
    database: &DatabaseConnection,
) -> Result<bool, DbErr> {
    let existing = Channels::find()
        .filter(ChannelsColumn::Uid.eq(user_id.get() as i64))
        .one(database)
        .await?;

    Ok(existing.is_some())
}

pub async fn db_delete_channel_by_cid(
    channel_id: ChannelId,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    Channels::delete_many()
        .filter(ChannelsColumn::Cid.eq(channel_id.get() as i64))
        .exec(database)
        .await?;
    Ok(())
}

use crate::entities::items::Model;

pub async fn db_list_items(
    channel_id: ChannelId,
    database: &DatabaseConnection,
) -> Result<Vec<Model>, DbErr> {
    let items = Items::find()
        .filter(ItemsColumn::OriginCid.eq(channel_id.get() as i64))
        .all(database)
        .await?;
    Ok(items)
}

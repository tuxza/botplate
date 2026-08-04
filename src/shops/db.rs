// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/channels/db.rs
use crate::entities::prelude::Channels;
use crate::entities::types::{ChannelsActiveModel, ChannelsColumn};
use poise::serenity_prelude::{ChannelId, UserId};
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

pub async fn db_create_channel(
    new_channel_id: ChannelId,
    user_id: UserId,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    crate::global::ensure_user_exists(user_id.get(), database).await?;

    let active_model = ChannelsActiveModel {
        cid: Set(new_channel_id.get()),
        uid: Set(user_id.get()),
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
    uid: u64,
    database: &DatabaseConnection,
) -> Result<Option<ChannelId>, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Uid.eq(uid))
        .one(database)
        .await?;
    Ok(channel.map(|c| ChannelId::new(c.cid)))
}

pub async fn db_get_shop_owner_id(
    cid: u64,
    database: &DatabaseConnection,
) -> Result<Option<UserId>, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Cid.eq(cid))
        .one(database)
        .await?;

    Ok(channel.map(|c| UserId::new(c.uid)))
}

pub async fn db_delete_shop(uid: UserId, database: &DatabaseConnection) -> Result<(), DbErr> {
    Channels::delete_many()
        .filter(ChannelsColumn::Uid.eq(uid.get()))
        .exec(database)
        .await?;
    Ok(())
}

pub async fn db_user_has_shop(uid: UserId, database: &DatabaseConnection) -> Result<bool, DbErr> {
    let existing = Channels::find()
        .filter(ChannelsColumn::Uid.eq(uid.get()))
        .one(database)
        .await?;

    Ok(existing.is_some())
}

pub async fn db_delete_channel_by_cid(
    channel_id: ChannelId,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    Channels::delete_many()
        .filter(ChannelsColumn::Cid.eq(channel_id.get()))
        .exec(database)
        .await?;
    Ok(())
}

pub async fn db_verify_shop(
    uid: u64,
    cid: u64,
    database: &DatabaseConnection,
) -> Result<bool, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Cid.eq(cid))
        .one(database)
        .await?;

    Ok(match channel {
        Some(c) => c.uid == uid,
        None => false,
    })
}

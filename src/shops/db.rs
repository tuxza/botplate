// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/channels/db.rs
use crate::entities::prelude::Channels;
use crate::entities::types::{ChannelsActiveModel, ChannelsColumn};
use poise::serenity_prelude::ChannelId;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

pub async fn db_create_channel(
    new_cid: i64,
    uid: i64,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    crate::global::ensure_user_exists(uid, database).await?;

    let active_model = ChannelsActiveModel {
        cid: Set(new_cid),
        uid: Set(uid),
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
    uid: i64,
    database: &DatabaseConnection,
) -> Result<Option<ChannelId>, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Uid.eq(uid))
        .one(database)
        .await?;
    Ok(channel.map(|c| ChannelId::new(c.cid.cast_unsigned())))
}

pub async fn db_get_shop_owner_id(
    cid: u64,
    database: &DatabaseConnection,
) -> Result<Option<i64>, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Cid.eq(cid.cast_signed()))
        .one(database)
        .await?;

    Ok(channel.map(|c| c.uid))
}

pub async fn db_delete_shop(uid: i64, database: &DatabaseConnection) -> Result<(), DbErr> {
    Channels::delete_many()
        .filter(ChannelsColumn::Uid.eq(uid))
        .exec(database)
        .await?;
    Ok(())
}

pub async fn db_user_has_shop(uid: i64, database: &DatabaseConnection) -> Result<bool, DbErr> {
    let existing = Channels::find()
        .filter(ChannelsColumn::Uid.eq(uid))
        .one(database)
        .await?;

    Ok(existing.is_some())
}

pub async fn db_delete_channel_by_cid(
    cid: i64,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    Channels::delete_many()
        .filter(ChannelsColumn::Cid.eq(cid))
        .exec(database)
        .await?;
    Ok(())
}

pub async fn db_verify_shop_owner(
    uid: u64,
    cid: u64,
    database: &DatabaseConnection,
) -> Result<bool, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Cid.eq(cid.cast_signed()))
        .one(database)
        .await?;

    Ok(match channel {
        Some(c) => c.uid == uid.cast_signed(),
        None => false,
    })
}

pub async fn db_verify_shop_exists(cid: u64, database: &DatabaseConnection) -> Result<bool, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Cid.eq(cid.cast_signed()))
        .one(database)
        .await?;

    Ok(channel.is_some())
}

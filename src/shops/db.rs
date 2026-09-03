// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/channels/db.rs
use crate::entities::prelude::Channels;
use crate::entities::types::{ChannelsActiveModel, ChannelsColumn};
use crate::types::{ChannelId64, UserId64};
use poise::serenity_prelude::{ChannelId, UserId};
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

pub async fn db_create_channel(
    new_cid: ChannelId,
    uid: UserId,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    crate::global::ensure_user_exists(uid, database).await?;

    let active_model = ChannelsActiveModel {
        cid: Set(ChannelId64::from(new_cid).get()),
        uid: Set(UserId64::from(uid).get()),
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
    uid: UserId,
    database: &DatabaseConnection,
) -> Result<Option<ChannelId>, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Uid.eq(uid.get()))
        .one(database)
        .await?;
    Ok(channel.map(|c| ChannelId::new(c.cid.cast_unsigned())))
}

pub async fn db_get_shop_owner_id(
    cid: ChannelId,
    database: &DatabaseConnection,
) -> Result<Option<i64>, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Cid.eq(cid.get()))
        .one(database)
        .await?;

    Ok(channel.map(|c| c.uid))
}

pub async fn db_delete_shop(uid: UserId, database: &DatabaseConnection) -> Result<(), DbErr> {
    Channels::delete_many()
        .filter(ChannelsColumn::Uid.eq(UserId64::from(uid)))
        .exec(database)
        .await?;
    Ok(())
}

pub async fn db_user_has_shop(uid: UserId, database: &DatabaseConnection) -> Result<bool, DbErr> {
    let existing = Channels::find()
        .filter(ChannelsColumn::Uid.eq(UserId64::from(uid)))
        .one(database)
        .await?;

    Ok(existing.is_some())
}

pub async fn db_delete_channel_by_cid(
    cid: ChannelId,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    Channels::delete_many()
        .filter(ChannelsColumn::Cid.eq(ChannelId64::from(cid)))
        .exec(database)
        .await?;
    Ok(())
}

pub async fn db_verify_shop_owner(
    uid: UserId,
    cid: ChannelId,
    database: &DatabaseConnection,
) -> Result<bool, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Cid.eq(cid.get()))
        .one(database)
        .await?;

    Ok(match channel {
        Some(c) => c.uid == UserId64::from(uid).get(),
        None => false,
    })
}

pub async fn db_verify_shop_exists(
    cid: ChannelId,
    database: &DatabaseConnection,
) -> Result<bool, DbErr> {
    let channel = Channels::find()
        .filter(ChannelsColumn::Cid.eq(cid.get()))
        .one(database)
        .await?;

    Ok(channel.is_some())
}

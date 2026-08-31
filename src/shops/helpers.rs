// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/shops/helpers.rs

use crate::errors::Error;
use crate::shops::db::{
    db_create_channel, db_delete_shop, db_get_shop_channel_id, db_user_has_shop,
};
use poise::serenity_prelude as serenity;
use sea_orm::DatabaseConnection;
use serenity::builder::CreateChannel;
use serenity::http::Http;
use serenity::model::channel::{
    ChannelType, GuildChannel, PermissionOverwrite, PermissionOverwriteType,
};
use serenity::model::id::{ChannelId, GuildId, UserId};
use serenity::model::permissions::Permissions;
use std::collections::HashMap;

pub async fn create_shop(
    http: &Http,
    guild_id: GuildId,
    uid: UserId,
    channel_name: String,
    database: &DatabaseConnection,
) -> Result<ChannelId, Error> {
    if db_user_has_shop(uid.get().cast_signed(), database).await? {
        return Err(Error::Custom("you already own a shop!".into()));
    }
    let channels = guild_id.channels(http).await?;
    let category_id = check_category(http, guild_id, &channels, "shops").await?;
    let user_overwrites = PermissionOverwrite {
        allow: Permissions::MANAGE_CHANNELS | Permissions::VIEW_CHANNEL,
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(uid),
    };
    let mut create_channel = CreateChannel::new(&channel_name)
        .kind(ChannelType::Text)
        .permissions(vec![user_overwrites]);
    if let Some(cat_id) = category_id {
        create_channel = create_channel.category(cat_id);
    }
    let new_channel = guild_id.create_channel(http, create_channel).await?;
    db_create_channel(
        new_channel.id.get().cast_signed(),
        uid.get().cast_signed(),
        database,
    )
    .await?;
    Ok(new_channel.id)
}

pub async fn delete_shop(
    http: &Http,
    uid: UserId,
    database: &DatabaseConnection,
) -> Result<(), Error> {
    let audit_log_reason = "Deleted by {uid}";

    let Some(channel_id) = db_get_shop_channel_id(uid.get().cast_signed(), database).await? else {
        return Err(Error::Custom("you don't own a shop!".into()));
    };

    db_delete_shop(uid.get().cast_signed(), database).await?;
    http.delete_channel(channel_id, Some(audit_log_reason))
        .await?;

    Ok(())
}

pub async fn rename_shop(http: &Http, channel_id: ChannelId, new_name: &str) -> Result<(), Error> {
    let builder = serenity::EditChannel::new().name(new_name);
    if let Err(why) = channel_id.edit(&http, builder).await {
        return Err(Error::Discord(Box::new(why)));
    }
    Ok(())
}

// i thought to myself to maybe call this on startup, and THEN have a global struct that holds a bool or something,
// but.. this is literally an API call and.. i dont really think it matters too much. someone can OBVIOUSLY
// correct me on that but.. whatevs!
pub async fn check_category(
    http: &Http,
    guild_id: GuildId,
    channels: &HashMap<ChannelId, GuildChannel>,
    category_name: &str,
) -> Result<Option<ChannelId>, serenity::Error> {
    let mut category_id = channels
        .values()
        .find(|ch| ch.kind == ChannelType::Category && ch.name.eq_ignore_ascii_case(category_name))
        .map(|ch| ch.id);

    if category_id.is_none() {
        let create_cat_builder = CreateChannel::new(category_name).kind(ChannelType::Category);
        let new_cat = guild_id.create_channel(http, create_cat_builder).await?;
        category_id = Some(new_cat.id);
    }

    Ok(category_id)
}

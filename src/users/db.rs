// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/users/helpers.rs

use crate::entities::prelude::Users;
use crate::entities::types::UsersActiveModel;
use crate::entities::types::UsersColumn;
use crate::global::ensure_user_exists;
use crate::types::TuxBux;
use crate::types::UserId64;
use dashmap::DashMap;
use poise::serenity_prelude::UserId;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    sea_query::Expr, sea_query::OnConflict,
};

/// Fetches a Unix timestamp of when the user last claimed their daily reward.
///
/// # Arguments
///
/// * `uid` - The 64-bit Discord ID (or general user ID) of the target user.
/// * `database` - A reference to the active [`DatabaseConnection`].
///
/// # Returns
///
/// * `Some(i64)` - Unix timestamp (in seconds) if the user exists and has claimed a daily before.
/// * `None` - If the user has never claimed a daily reward or doesn't exist yet.
///
/// # Examples
///
/// ```rust
/// let timestamp = db_last_daily(123456789012345678, &db).await;
/// if let Some(ts) = timestamp {
///     println!("Last claimed at: {ts}");
/// }
/// ```
pub async fn db_last_daily(uid: UserId, database: &DatabaseConnection) -> Option<i64> {
    Users::find_by_id(uid)
        .one(database)
        .await
        .ok()
        .flatten()
        .and_then(|u| u.last_daily)
}

/// Checks whether a user is eligible to claim a daily reward based on their last claim timestamp.
///
/// Requires 24 hours (86,400 seconds) to have elapsed since the previous claim.
///
/// # Arguments
///
/// * `last_daily` - An `Option<i64>` containing the Unix timestamp of the last claim.
///
/// # Returns
///
/// * `true` - If `last_daily` is `None` or if 24+ hours have passed.
/// * `false` - If the cooldown is still active.
pub fn can_claim_daily(last_daily: Option<i64>) -> bool {
    let Some(last_claim) = last_daily else {
        return true;
    };

    let now = chrono::Utc::now().timestamp();
    now - last_claim >= 86_400
}

/// Sets the last daily claim timestamp for a user in the database.
///
/// # Arguments
///
/// * `user_id` - The ID of the user.
/// * `timestamp` - The Unix timestamp of the last claim.
/// * `database` - The database connection.
///
/// # Returns
///
/// * it returns nothing, lol.. i should fix that.
pub async fn db_set_last_daily(uid: UserId, timestamp: i64, database: &DatabaseConnection) {
    let active_model = UsersActiveModel {
        id: Set(uid.into()),
        last_daily: Set(Some(timestamp)),
        ..Default::default()
    };

    let _ = Users::insert(active_model)
        .on_conflict(
            OnConflict::column(UsersColumn::Id)
                .update_column(UsersColumn::LastDaily)
                .to_owned(),
        )
        .exec(database)
        .await;
}

/// Gets the balance of a user from the database.
///
/// # Arguments
///
/// * `user_id` - The ID of the user.
/// * `database` - The database connection.
///
/// # Returns
///
/// The user's balance as an `i64` value.
pub async fn db_get_balance(uid: UserId, database: &DatabaseConnection) -> i64 {
    Users::find_by_id(UserId64::from(uid))
        .one(database)
        .await
        .ok()
        .flatten()
        .map_or(0, |u| u.tokens)
}

/// Adjusts a user's token balance atomically.
///
/// If the user does not exist in the database, a new record is created with `amount`
/// as their starting balance. If they already exist, `amount` is added to (or subtracted from)
/// their current balance.
///
/// # Arguments
///
/// * `user_id` - The 64-bit user ID.
/// * `amount` - The number of tokens to add (positive) or deduct (negative).
/// * `database` - A reference to the active [`DatabaseConnection`].
///
/// # Examples
///
/// ```ignore
/// // Add 500 tokens
/// db_add_balance(123456789, 500, &db).await;
///
/// // Deduct 150 tokens
/// db_add_balance(123456789, TuxBux(-150), &db).await;
/// ```
pub async fn db_add_balance(
    uid: UserId,
    amount: TuxBux,
    database: &DatabaseConnection,
) -> Result<(), DbErr> {
    ensure_user_exists(uid, database).await?;

    Users::update_many()
        .col_expr(
            UsersColumn::Tokens,
            Expr::col(UsersColumn::Tokens).add(amount.0),
        )
        .filter(UsersColumn::Id.eq(uid.get()))
        .exec(database)
        .await?;

    Ok(())
}

pub async fn db_deduct_balance(
    uid: UserId,
    amount: TuxBux,
    database: &DatabaseConnection,
) -> Result<bool, DbErr> {
    ensure_user_exists(uid, database).await?;
    let result = Users::update_many()
        .col_expr(
            UsersColumn::Tokens,
            Expr::col(UsersColumn::Tokens).sub(amount.0),
        )
        .filter(UsersColumn::Id.eq(uid.get()))
        .filter(UsersColumn::Tokens.gte(amount.0))
        .exec(database)
        .await?;

    Ok(result.rows_affected > 0)
}

/// Returns the user's XP and level.
///
/// Checks the in-memory XP cache first; falls back to the database on a
/// cache miss (e.g. bot just restarted, or the user hasn't sent a message
/// since the cache was last populated) and populates the cache from that
/// result so future lookups skip the DB.
///
/// Returns `(0, 0)` if the user does not exist in either the cache or DB.
pub async fn get_user_xp_and_level(
    uid: UserId,
    user_map: &DashMap<UserId, (i64, i64, i64)>,
    database: &DatabaseConnection,
) -> Result<(i64, i64, i64), DbErr> {
    if let Some(entry) = user_map.get(&uid) {
        return Ok(*entry);
    }

    let user = Users::find_by_id(uid).one(database).await?;
    let rank = user.map_or((0, 0, 0), |u| (u.xp, u.level, u.tokens));

    user_map.insert(uid, rank);

    Ok(rank)
}

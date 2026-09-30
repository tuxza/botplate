// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/users/helpers.rs

use crate::entities::prelude::Users;
use crate::entities::types::UsersColumn;
use crate::global::ensure_user_exists;
use crate::types::{MAX_BALANCE, TuxBux, UserId64};
use dashmap::DashMap;
use poise::serenity_prelude::UserId;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, sea_query::Expr,
    sea_query::ExprTrait,
};

// Claims a daily reward if the previous claim was at least 24 hours ago.
//
// # Arguments
//
// * [`UserId`] - The ID of the user.
// * `database` - A reference to the active [`DatabaseConnection`].
//
// # Returns
//
// * `Some(timestamp)` if the claim was recorded.
// * `None` if the user does not exist or the cooldown hasn't finished.
pub async fn claim_daily(uid: UserId, database: &DatabaseConnection) -> Result<Option<i64>, DbErr> {
    ensure_user_exists(uid, database).await?;

    let now = chrono::Utc::now().timestamp();
    let cutoff = now - 86_400;
    let user_id = UserId64::from(uid).get();

    let updated = Users::update_many()
        .col_expr(UsersColumn::LastDaily, Expr::value(now))
        .filter(UsersColumn::Id.eq(user_id))
        .filter(
            UsersColumn::LastDaily
                .is_null()
                .or(UsersColumn::LastDaily.lte(cutoff)),
        )
        .exec(database)
        .await?;

    if updated.rows_affected > 0 {
        return Ok(None);
    }

    let last_daily = Users::find_by_id(user_id)
        .one(database)
        .await?
        .and_then(|user| user.last_daily);

    Ok(last_daily.map(|last| last + 86_400))
}

/// Gets the balance of a user from the database.
///
/// # Arguments
///
/// * [`UserId`] - The ID of the user.
/// * `database` - A reference to the active [`DatabaseConnection`].
///
/// # Returns
///
/// The user's balance as a [`TuxBux`] value.
pub async fn db_get_balance(uid: UserId, database: &DatabaseConnection) -> Result<TuxBux, DbErr> {
    let balance = Users::find_by_id(UserId64::from(uid).get())
        .one(database)
        .await?
        .map_or(TuxBux(0), |u| TuxBux(u.tokens));

    Ok(balance)
}

/// Adjusts a user's token balance atomically.
///
/// If the user does not exist in the database, a new record is created with `amount`
/// as their starting balance. If they already exist, `amount` is added to their current balance.
///
/// # Arguments
///
/// * [`UserId`] - User ID
/// * `amount` - The amount of `TuxBux` to add.
/// * `database` - A reference to the active [`DatabaseConnection`].
///
/// # Examples
///
/// ```ignore
/// // Add 500 tokens
/// db_add_balance(UserId, TuxBux(500), &db).await;
///
/// Returns `true` if the balance was successfully updated, `false` otherwise.
pub async fn db_add_balance(
    uid: UserId,
    amount: TuxBux,
    database: &DatabaseConnection,
) -> Result<bool, DbErr> {
    if amount.0 <= 0 {
        return Err(DbErr::Custom(
            "db_add_balance called with a non-positive amount. its not supposed to do that btw"
                .to_string(),
        ));
    }

    ensure_user_exists(uid, database).await?;

    let result = Users::update_many()
        .col_expr(
            UsersColumn::Tokens,
            Expr::case(
                Expr::col(UsersColumn::Tokens).add(amount.0).gt(MAX_BALANCE),
                MAX_BALANCE,
            )
            .finally(Expr::col(UsersColumn::Tokens).add(amount.0))
            .into(),
        )
        .filter(UsersColumn::Id.eq(uid.get()))
        .exec(database)
        .await?;

    Ok(result.rows_affected > 0)
}

/// Deducts a user's token balance atomically.
///
/// # Arguments
///
/// * `UserId` - User ID, a u64.
/// * `amount` - The amount of `TuxBux` to add.
/// * `database` - A reference to the active [`DatabaseConnection`].
///
/// # Examples
///
/// ```ignore
/// // Add 500 tokens
/// db_deduct_balance(UserId, TuxBux(500), &db).await;
///
/// Returns `true` if the balance was successfully deducted, `false` otherwise.
pub async fn db_deduct_balance(
    uid: UserId,
    amount: TuxBux,
    database: &DatabaseConnection,
) -> Result<bool, DbErr> {
    if amount.0 <= 0 {
        return Err(DbErr::Custom(
            "db_deduct_balance called with a positive amount, dude these developers SUCK"
                .to_string(),
        ));
    }

    ensure_user_exists(uid, database).await?;
    let result = Users::update_many()
        .col_expr(
            UsersColumn::Tokens,
            Expr::col(UsersColumn::Tokens).sub(amount.0),
        )
        .filter(UsersColumn::Id.eq(UserId64::from(uid)))
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
    ensure_user_exists(uid, database).await?;
    if let Some(entry) = user_map.get(&uid) {
        return Ok(*entry);
    }

    let user = Users::find_by_id(uid).one(database).await?;
    let rank = user.map_or((0, 0, 0), |u| (u.xp, u.level, u.message_count));

    user_map.insert(uid, rank);

    Ok(rank)
}

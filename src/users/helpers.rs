/* This is the helpers.rs file. This file containers helper functions for user related operations
 * to call any of these functions, pass their related arguments.
 * To contribute to botplate, please read README.md in this directory.
 */

use crate::entities;
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait, sea_query::OnConflict};

/// Fetches a Unix timestamp of when the user last claimed their daily reward.
///
/// # Arguments
///
/// * `user_id` - The 64-bit Discord ID (or general user ID) of the target user.
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
/// let timestamp = last_daily(123456789012345678, &db).await;
/// if let Some(ts) = timestamp {
///     println!("Last claimed at: {ts}");
/// }
/// ```
pub async fn last_daily(user_id: i64, database: &DatabaseConnection) -> Option<i64> {
    entities::users::Entity::find_by_id(user_id)
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

pub async fn set_last_daily(user_id: i64, timestamp: i64, database: &DatabaseConnection) {
    let active_model = entities::users::ActiveModel {
        id: Set(user_id),
        last_daily: Set(Some(timestamp)),
        ..Default::default()
    };

    let _ = entities::users::Entity::insert(active_model)
        .on_conflict(
            OnConflict::column(entities::users::Column::Id)
                .update_column(entities::users::Column::LastDaily)
                .to_owned(),
        )
        .exec(database)
        .await;
}

pub async fn get_balance(user_id: i64, database: &DatabaseConnection) -> i64 {
    entities::users::Entity::find_by_id(user_id)
        .one(database)
        .await
        .ok()
        .flatten()
        .map(|u| u.tokens.abs())
        .unwrap_or(0)
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
/// edit_balance(123456789, 500, &db).await;
///
/// // Deduct 150 tokens
/// edit_balance(123456789, -150, &db).await;
/// ```
pub async fn edit_balance(user_id: i64, amount: i64, database: &DatabaseConnection) {
    use sea_orm::sea_query::Expr;

    let new_user = entities::users::ActiveModel {
        id: Set(user_id),
        tokens: Set(amount),
        debt: Set(0),
        last_daily: Set(None),
        last_job: Set(None),
        xp: Set(0),
        level: Set(0),
    };

    let _ = entities::users::Entity::insert(new_user)
        .on_conflict(
            OnConflict::column(entities::users::Column::Id)
                .value(
                    entities::users::Column::Tokens,
                    Expr::col(entities::users::Column::Tokens).add(amount),
                )
                .to_owned(),
        )
        .exec(database)
        .await;
}

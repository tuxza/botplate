// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/global.rs

use poise::serenity_prelude::CreateEmbedFooter;
use rand::prelude::IndexedRandom;

use crate::entities::prelude::Users;
use crate::entities::types::{UsersActiveModel, UsersColumn};
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};

/// Ensures a user row exists in the database, creating one with a
/// starting balance, debt, xp, etc. of 0 if it doesn't. No-op if the user already exists.
pub async fn ensure_user_exists(uid: u64, database: &DatabaseConnection) -> Result<(), DbErr> {
    let active_model = UsersActiveModel {
        id: Set(uid.cast_signed()),
        tokens: Set(0),
        debt: Set(0),
        last_daily: Set(None),
        last_job: Set(None),
        xp: Set(0),
        level: Set(0),
        spouse: Set(None),
        spouse_since: Set(None),
        joint_balance: Set(None),
    };

    Users::insert(active_model)
        .on_conflict(OnConflict::column(UsersColumn::Id).do_nothing().to_owned())
        .exec_without_returning(database)
        .await?;

    Ok(())
}

// this could be a lot more elegant and nice to work with, but.. im a lazy ass.

pub fn is_admin(author: u64, admins: u64) -> bool {
    author == admins
}

pub async fn make_numbers_pretty(num: i64) -> String {
    let s = num.to_string();
    let mut result = String::new();

    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }

    result.chars().rev().collect()
}

pub async fn random_footer() -> CreateEmbedFooter {
    let mut rng = rand::rng();
    let version = env!("CARGO_PKG_VERSION");
    let messages = [
        "botplate-rs is cool",
        "check out our github repo!",
        "how random is random..?",
        "tuxzilla is in your walls",
        "yo yall seen those creepy footers??",
        "FOOTER",
        "dude why are you reading this",
        "duckie please stop dming me",
        "billions of tuxaroos",
        "billions must love",
        "wait what is this server again",
        "tuxzilla vs making a good bot",
        "mold -run cargo build --release",
    ];
    let Some(message) = messages.choose(&mut rng) else {
        return CreateEmbedFooter::new(format!("botplate-rs reimagined | {version}"));
    };
    CreateEmbedFooter::new(format!("{message} | botplate-rs reimagined | {version}"))
}

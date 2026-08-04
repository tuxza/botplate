// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/users/user.rs

use crate::errors::Error;
use crate::users::helpers;

/// check your balance
#[poise::command(prefix_command, slash_command)]
pub async fn balance(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    let author = ctx.author();
    let balance = helpers::get_balance(author.id.get(), &ctx.data().database).await;
    let _ = ctx.say(format!("Your balance is: {balance}")).await?;
    Ok(())
}

/// claim your daily tuxbux
#[poise::command(prefix_command, slash_command)]
pub async fn daily(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    let author = ctx.author();
    let last = helpers::last_daily(author.id.get(), &ctx.data().database).await;

    if !helpers::can_claim_daily(last) {
        ctx.say("you already claimed your daily today! come back later please.")
            .await?;
        return Ok(());
    }

    helpers::edit_balance(author.id.get(), 100, &ctx.data().database).await?; // tux reminder: make this configurable because your so nice
    helpers::set_last_daily(
        author.id.get(),
        chrono::Utc::now().timestamp(),
        &ctx.data().database,
    )
    .await;

    ctx.say("Claimed 100 tokens!").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn rank(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    let author = ctx.author();
    let (xp, level) =
        helpers::get_user_xp_and_level(author.id.get(), &ctx.data().xp_map, &ctx.data().database)
            .await?;

    ctx.say(format!("You are level {level} with {xp} XP."))
        .await?;

    Ok(())
}

// now i THOUGHT of putting the work that gamble does into a function
// but.. i kind thought it wouldnt be used again..
// obviously get_balance is used frequently LOL
// but how OFTEN are you gambling that you need a whole ass function for it??

/// gamble tuxbux with a 50/50 chance of winning or losing
#[poise::command(prefix_command, slash_command)]
pub async fn gamble(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "how many tuxbux to gamble"] amount: i64,
) -> Result<(), Error> {
    let author = ctx.author();
    let balance = helpers::get_balance(author.id.get(), &ctx.data().database).await;
    if amount <= 0 {
        ctx.say("you gotta gamble a positive amount, dummy.")
            .await?;
        return Ok(());
    }
    if balance < amount {
        ctx.say(format!(
            "you don't have {amount} tuxbux to gamble! your balance: {balance}"
        ))
        .await?;
        return Ok(());
    }
    let won = rand::random::<bool>();
    if won {
        helpers::edit_balance(author.id.get(), amount, &ctx.data().database).await?;
        ctx.say(format!("you won! +{amount} tuxbux")).await?;
    } else {
        helpers::edit_balance(author.id.get(), -amount, &ctx.data().database).await?;
        ctx.say(format!("you lost! -{amount} tuxaroos")).await?;
    }
    Ok(())
}

// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/users/user.rs

use poise::CreateReply;
use poise::serenity_prelude::{CreateEmbed, Mentionable, UserId};

use crate::errors::Error;
use crate::global::{make_numbers_pretty, random_footer};
use crate::users::helpers;

/// check your balance
#[poise::command(prefix_command, slash_command)]
pub async fn balance(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "user id to check"] user: Option<UserId>,
) -> Result<(), Error> {
    let uid = user.unwrap_or(ctx.author().id);

    let user = uid.to_user(ctx).await?;

    let display_name = user.global_name.as_deref().unwrap_or(&user.name);

    let balance = helpers::db_get_balance(uid.get(), &ctx.data().database).await;
    let balance = make_numbers_pretty(balance);
    let embed = CreateEmbed::new()
        .title(format!("Balance of {display_name}"))
        .description(format!("{} has {balance} tuxbux.", uid.mention()))
        .color(0x7289DA)
        .footer(random_footer());

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

/// claim your daily tuxbux
#[poise::command(prefix_command, slash_command)]
pub async fn daily(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    let author = ctx.author();
    let last = helpers::db_last_daily(author.id.get(), &ctx.data().database).await;

    if !helpers::can_claim_daily(last) {
        ctx.say("you already claimed your daily today! come back later please.")
            .await?;
        return Ok(());
    }

    helpers::db_edit_balance(author.id.get(), 100, &ctx.data().database).await?; // tux reminder: make this configurable because your so nice
    helpers::db_set_last_daily(
        author.id.get(),
        chrono::Utc::now().timestamp(),
        &ctx.data().database,
    )
    .await;

    ctx.say("Claimed 100 tokens!").await?;
    Ok(())
}

/// Check your rank and XP.
#[poise::command(prefix_command, slash_command)]
pub async fn rank(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "User to look at"] user_id: Option<UserId>,
) -> Result<(), Error> {
    let uid = user_id.unwrap_or(ctx.author().id);

    let user = uid.to_user(ctx).await?;

    let display_name = user.global_name.as_deref().unwrap_or(&user.name);

    let (xp, level, _) =
        helpers::get_user_xp_and_level(uid.get(), &ctx.data().user_map, &ctx.data().database)
            .await?;

    let embed = CreateEmbed::new()
        .title(format!("Rank for {display_name}"))
        .description(format!("{} is Level {level} with {xp} XP.", uid.mention()))
        .color(0x7289DA)
        .footer(random_footer());

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

// now i THOUGHT of putting the work that gamble does into a helper function
// but.. i kind thought it wouldnt be used again..
// obviously get_balance is used frequently LOL
// but how OFTEN are you gambling that you need a whole ass function for it??

/// gamble tuxbux with a 50/50 chance of winning or losing
#[poise::command(prefix_command, slash_command)]
pub async fn gamble(
    ctx: poise::Context<'_, crate::Data, Error>,
    #[description = "how many tuxbux to gamble"] wager: i64,
) -> Result<(), Error> {
    let author = ctx.author();
    let deducted = helpers::db_try_deduct(author.id.get(), wager, &ctx.data().database).await?;
    if !deducted {
        ctx.say(format!("you don't have {wager} tuxbux to gamble!"))
            .await?;
        return Ok(());
    }

    let won = rand::random::<bool>();
    if won {
        let payout = wager * 2;
        helpers::db_edit_balance(author.id.get(), payout, &ctx.data().database).await?;
        ctx.say(format!("you won! +{wager} tuxbux")).await?;
    } else {
        ctx.say(format!("you lost! -{wager} tuxaroos")).await?;
    }
    Ok(())
}

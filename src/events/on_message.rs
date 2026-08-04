// src/events/event_handler.rs (or wherever on_message lands)

use dashmap::DashMap;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Mentionable;
use rand::RngExt;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::entities::prelude::Users;
use crate::entities::types::UsersActiveModel;
use crate::errors::Error;

// I HATED WRITING THIS AND I HOPE IT DIES

pub async fn on_message(
    new_message: &serenity::Message,
    ctx: &serenity::Context,
    xp_map: &DashMap<i64, (i64, i64)>,
    database: &DatabaseConnection,
    xp_per_level: i64,
) -> Result<(), Error> {
    if new_message.author.bot {
        return Ok(());
    }

    let uid = new_message.author.id.get() as i64;

    // if the user is not in the map, fetch their xp/level from the database. so yk..
    // we dont have to ask the DB everytime someone sends a message.
    // i (hope) this is faster than querying the database. (i hope.)
    if !xp_map.contains_key(&uid) {
        let (xp, level) = Users::find_by_id(uid)
            .one(database)
            .await?
            .map_or((0, 0), |u| (u.xp, u.level));
        xp_map.insert(uid, (xp, level));
    }

    let xp_gained = rand::rng().random_range(5..=15);
    let mut leveled_up = false;
    let new_xp: i64;
    let new_level: i64;

    {
        let mut entry = xp_map
            .get_mut(&uid)
            .expect("just added or already present #fuck");
        entry.0 += xp_gained;

        let next_level_xp = xp_per_level * (entry.1 + 1);
        if entry.0 >= next_level_xp {
            entry.1 += 1;
            entry.0 -= next_level_xp;
            leveled_up = true;
        }

        new_xp = entry.0;
        new_level = entry.1;
    }

    if leveled_up {
        level_up(uid, new_xp, new_level, database).await?;

        let tokens_earned = rand::rng().random_range(1..=100);

        let embed = serenity::CreateEmbed::new()
            .title("level up!")
            .description(format!(
                "{} has reached level **{}**.",
                new_message.author.mention(),
                new_level
            ))
            .field(
                "tuxbux earned",
                format!("**{tokens_earned} tuxbux**"),
                true,
            )
            .color(0xFFD700);

        new_message
            .channel_id
            .send_message(&ctx.http, serenity::CreateMessage::new().embed(embed))
            .await?;

        crate::users::helpers::edit_balance(uid, tokens_earned, database).await?;
    }

    Ok(())
}

// now CONTRIBUTING.md would tell you to put this in db.rs
// but i dont follow my own rules.

async fn level_up(
    uid: i64,
    xp: i64,
    level: i64,
    database: &DatabaseConnection,
) -> Result<(), Error> {
    let active_model = UsersActiveModel {
        id: Set(uid),
        xp: Set(xp),
        level: Set(level),
        ..Default::default()
    };

    active_model.update(database).await?;
    Ok(())
}

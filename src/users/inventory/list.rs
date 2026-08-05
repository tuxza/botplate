// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/users/inventory/list.rs

use crate::global::{escape_md, make_numbers_pretty, random_footer, truncate};
use crate::errors::Error;
use poise::serenity_prelude::CreateEmbed;

const MAX_EMBED_FIELDS: usize = 25;
const MAX_FIELD_VALUE: usize = 1000;

/// check your inventory
#[poise::command(prefix_command, slash_command)]
pub async fn inventory(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    let author = ctx.author();
    let items =
        crate::users::inventory::db::db_get_inventory(author.id.get() as i64, &ctx.data().database)
            .await?;

    if items.is_empty() {
        ctx.say("your inventory is empty!").await?;
        return Ok(());
    }

    let lines: Vec<String> = items
        .iter()
        .map(|(name, qty)| {
            format!(
                "**{}** x{}",
                truncate(&escape_md(name), 64),
                make_numbers_pretty(*qty)
            )
        })
        .collect();

    // one field per ~1000 chars rather than one field for everything: a field
    // value caps at 1024 and a big inventory used to blow past it, failing the
    // whole message.
    let mut chunks: Vec<String> = Vec::new();
    let mut current = String::new();

    for line in lines {
        if !current.is_empty()
            && current.chars().count() + line.chars().count() + 1 > MAX_FIELD_VALUE
        {
            chunks.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push('\n');
        }
        current.push_str(&line);
    }
    if !current.is_empty() {
        chunks.push(current);
    }

    let truncated = chunks.len() > MAX_EMBED_FIELDS;
    chunks.truncate(MAX_EMBED_FIELDS);

    let mut embed = CreateEmbed::new().color(0x7492B9).title("your inventory");

    for (i, chunk) in chunks.iter().enumerate() {
        let heading = if i == 0 {
            "items".to_string()
        } else {
            format!("items (cont. {})", i + 1)
        };
        embed = embed.field(heading, chunk.as_str(), false);
    }

    if truncated {
        embed = embed.description("you have more items than fit in one message.");
    }

    let embed = embed.footer(random_footer());

    let reply = poise::CreateReply::default().embed(embed);

    ctx.send(reply).await?;

    Ok(())
}

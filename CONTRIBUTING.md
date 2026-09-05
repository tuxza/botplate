# Contributing to botplate

Thanks for taking an interest in botplate!

Below is a guide from the creator, [Tuxzilla](https://git.tuxzilla.com/tuxza) on how to work with botplate.

Just some simple rules, Nothin' crazy.

## Ground rules

- **Target the `testing` branch**, not `master`. Active development happens on
  `testing`; `master` lags behind and gets updated in batches whenever Tux gets moody enough.
- Open an issue for bugs and small fixes/typos. Full feature changes should be a PR.
- One logical change per PR. Please don't bundle an unrelated refactor with a
  bugfix — makes review harder and history messier. (#notokenmaxxing)
- Run `cargo clippy` and `cargo fmt` before opening a PR.
- If your change touches the database, please make sure it works. sea-orm is a complex beast, so test test test!

## Project structure

The codebase is organized by feature:

```
src/
  <feature>/
    <feature>.rs   -- the actual poise commands
    helpers.rs     -- logic lives here!
    db.rs          -- (where relevant) database-specific helpers
    README.md      -- short index of what's in a directory
  migration/       -- SeaORM migrations
  entities/        -- SeaORM entities
    types.rs       -- database related types
  events/          -- Discord event handlers
  errors.rs        -- shared Error types (please use these..)
  global.rs        -- misc shared helpers
  types.rs         -- type wrappers for NOT database types
  main.rs          -- entry file
```

### Commands stay thin, helpers do the work.

This is a pattern throughout the project: a `#[poise::command]`
function should mostly just parse arguments, call into `helpers.rs`, and format
a reply. Business logic — math, cooldown checks, validation, etc. — belongs
in the helper functions so it's reusable outside of a Discord context. (WebUIs, If I may?)

```rust
// good example
#[poise::command(prefix_command, slash_command)]
pub async fn daily(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    let uid = ctx.author().id;
    let last_daily = db::db_last_daily(uid, &ctx.data().database)
        .await
        .unwrap_or(0);

    if !db::can_claim_daily(Some(last_daily)) {
        let next_daily = last_daily + 86_400;
        ctx.say(format!(
            "you already claimed your daily today! come back <t:{next_daily}:R>"
        ))
        .await?;
        return Ok(());
    }
    
    db::db_add_balance(uid, TuxBux(100), &ctx.data().database).await?;
    db::db_set_last_daily(uid, chrono::Utc::now().timestamp(), &ctx.data().database).await;

    ctx.say("Claimed 100 tokens!").await?;
    Ok(())
}
```

# Code Styling

I'm not too strict on how your code should LOOK (beyond the splitting everything into files.) But I DO have opinions on how it ACTS.

- Use the types provided in the root of the src directory
- I don't want to see a comment every 5 lines, one, we're not corporate devs, two, your code should be self-explanatory.

beyond that, I'm pretty lax.

## Commit / PR style

- Descriptive commit messages > cute ones. You can add a bit of ✨pazzaz✨ to your commit messages.. but please keep it concise and to the point.
- In the PR description, call out:
  - what your commit changes, fixes, or diverges from
  - anything you're unsure about / want a second opinion on
  - any context for your changes, or if an LLM was used.
- You generally won't need to bump version numbers yourself, Anything pulled to master gets a version bump.

## Development Setup

To set up the development environment, follow these steps:

I'll assume you already have rustup installed, If not:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

(assuming you're using a Unix like system. If not, see [rustup's installation guide](https://rustup.rs/))

1. Clone the repository:

```sh
git clone https://git.tuxzilla.com/tuxza/botplate.git
```

2. Install dependencies:

```sh
cargo build
```

3. Read this guide!

Wow, look at you! You're already doing that!

4. Create a local database

We use SQLite, so just name a file 'botplate.db' (or whatever.) and point your .env file to it.

(if you need an example of an .env file, see [.env.example](https://github.com/tuxza/botplate/blob/master/.env.example))

5. Run SeaORM migrations against the DB:

```sh
sea-orm-cli migrate up --database-url sqlite://botplate.db?mode=rwc
```

6. Start working!

## Questions?

Open an issue or ask our [Discord Server](https://tuxzilla.com/invite). We're not elitists.

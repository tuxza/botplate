use std::fmt;

#[derive(Debug)]
pub enum Error {
    Database(sea_orm::DbErr),
    Discord(poise::serenity_prelude::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Database(e) => write!(
                f,
                "database error: {e} please report this [here](https://tuxzilla.com/squash-a-bug) through proper channels"
            ),
            Error::Discord(e) => write!(
                f,
                "discord error: {e} please report this [here](https://tuxzilla.com/squash-a-bug) through proper channels"
            ),
        }
    }
}

impl std::error::Error for Error {}

impl From<sea_orm::DbErr> for Error {
    fn from(e: sea_orm::DbErr) -> Self {
        Error::Database(e)
    }
}

impl From<poise::serenity_prelude::Error> for Error {
    fn from(e: poise::serenity_prelude::Error) -> Self {
        Error::Discord(e)
    }
}

// use poise::serenity_prelude as serenity;
// use serenity::FullEvent; storage for later
use crate::entities;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub async fn add_tuxzilla(db: &DatabaseConnection) {
    let me = entities::users::ActiveModel {
        id: Set(1079199944738615367),
        tokens: Set(67),
        ..Default::default()
    };

    me.insert(db).await.unwrap();
}

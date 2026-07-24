// src/entities/types.rs
// this file is ONLY for hand-written type aliases for sea-orm entities to make the code cleaner
// kept separate from prelude since that file is sea-orm managed and would get overwritten.
use super::prelude::Users;
use sea_orm::EntityTrait;

pub type UsersActiveModel = <Users as EntityTrait>::ActiveModel;
pub type UsersColumn = <Users as EntityTrait>::Column;

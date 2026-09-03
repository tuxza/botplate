// types.rs
//

pub use poise::serenity_prelude::{ChannelId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId64(pub i64);

impl From<UserId> for UserId64 {
    fn from(uid: UserId) -> Self {
        UserId64(uid.get().cast_signed())
    }
}

impl From<UserId64> for i64 {
    fn from(id: UserId64) -> Self {
        id.0
    }
}

impl UserId64 {
    pub fn get(self) -> i64 {
        self.0
    }
}

impl From<UserId64> for sea_orm::Value {
    fn from(id: UserId64) -> Self {
        id.0.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChannelId64(pub i64);

impl From<ChannelId> for ChannelId64 {
    fn from(cid: ChannelId) -> Self {
        ChannelId64(cid.get().cast_signed())
    }
}

impl From<ChannelId64> for i64 {
    fn from(cid: ChannelId64) -> Self {
        cid.0.into()
    }
}

impl From<ChannelId64> for sea_orm::Value {
    fn from(id: ChannelId64) -> Self {
        id.0.into()
    }
}

impl ChannelId64 {
    pub fn get(self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(pub i64);

impl Quantity {
    pub fn get(self) -> i64 {
        self.0
    }
}

impl From<Quantity> for i64 {
    fn from(q: Quantity) -> Self {
        q.0
    }
}

impl From<Quantity> for sea_orm::Value {
    fn from(id: Quantity) -> Self {
        id.0.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TuxBux(pub i64);

impl std::fmt::Display for TuxBux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Mul<TuxBux> for TuxBux {
    type Output = TuxBux;

    fn mul(self, rhs: TuxBux) -> Self::Output {
        TuxBux(self.0 * rhs.0)
    }
}

impl From<TuxBux> for i64 {
    fn from(tb: TuxBux) -> Self {
        tb.0
    }
}

impl TuxBux {
    pub fn get(self) -> i64 {
        self.0
    }
}

impl From<i64> for TuxBux {
    fn from(amount: i64) -> Self {
        TuxBux(amount)
    }
}

impl From<TuxBux> for sea_orm::Value {
    fn from(tb: TuxBux) -> Self {
        tb.0.into()
    }
}

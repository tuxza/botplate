use poise::serenity_prelude as serenity;
use serenity::builder::CreateChannel;
use serenity::http::Http;
use serenity::model::channel::{ChannelType, PermissionOverwrite, PermissionOverwriteType};
use serenity::model::id::{ChannelId, GuildId, UserId};
use serenity::model::permissions::Permissions;

pub async fn create_new_shop(
    http: &Http,
    guild_id: GuildId,
    user_id: UserId,
    channel_name: String,
) -> Result<(), serenity::Error> {
    let category_id = check_category(http, guild_id, "shops").await?;

    let user_overwrites = PermissionOverwrite {
        allow: Permissions::MANAGE_CHANNELS | Permissions::VIEW_CHANNEL,
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(user_id),
    };

    let mut create_chan_builder = CreateChannel::new(&channel_name)
        .kind(ChannelType::Text)
        .permissions(vec![user_overwrites]);

    if let Some(cat_id) = category_id {
        create_chan_builder = create_chan_builder.category(cat_id);
    }

    guild_id.create_channel(http, create_chan_builder).await?;

    Ok(())
}

pub async fn check_category(
    http: &Http,
    guild_id: GuildId,
    category_name: &str,
) -> Result<Option<ChannelId>, serenity::Error> {
    let channels = guild_id.channels(http).await?;

    let mut category_id = channels
        .values()
        .find(|ch| ch.kind == ChannelType::Category && ch.name.eq_ignore_ascii_case(category_name))
        .map(|ch| ch.id);

    if category_id.is_none() {
        let create_cat_builder = CreateChannel::new(category_name).kind(ChannelType::Category);
        let new_cat = guild_id.create_channel(http, create_cat_builder).await?;
        category_id = Some(new_cat.id);
    }

    Ok(category_id)
}

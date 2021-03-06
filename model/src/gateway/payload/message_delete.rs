use crate::id::{ChannelId, MessageId, GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageDelete {
    pub channel_id: ChannelId,
    pub id: MessageId,
    pub guild_id: Option<GuildId>,
}

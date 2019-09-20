use crate::{
    guild::Role,
    id::GuildId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RoleCreate {
    pub guild_id: GuildId,
    pub role: Role,
}

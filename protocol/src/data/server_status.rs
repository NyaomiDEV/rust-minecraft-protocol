use crate::impl_json_encoder_decoder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::chat::Message;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ServerStatus {
    pub version: ServerVersion,
    pub players: OnlinePlayers,
    pub description: Message,
    pub favicon: Option<String>,
}

impl_json_encoder_decoder!(ServerStatus);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ServerVersion {
    pub name: String,
    pub protocol: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OnlinePlayers {
    pub max: u32,
    pub online: u32,
    #[serde(default)]
    pub sample: Vec<OnlinePlayer>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct OnlinePlayer {
    pub name: String,
    pub id: Uuid,
}

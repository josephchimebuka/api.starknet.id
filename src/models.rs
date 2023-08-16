use mongodb::Database;

use crate::config::Config;
use serde::Serialize;

pub struct AppState {
    pub conf: Config,
    pub db: Database,
}

#[derive(Serialize)]
pub struct Data {
    pub domain: String,
    pub addr: Option<String>,
    pub domain_expiry: Option<i32>,
    pub is_owner_main: bool,
    pub owner_addr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_github: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_discord: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_personhood: Option<String>,
    pub starknet_id: String,
}

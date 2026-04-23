use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NotificationPreferences {
    pub creator_username: String,
    pub notify_on_tip: bool,
    pub notify_on_milestone: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub notify_on_tip: Option<bool>,
    pub notify_on_milestone: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub creator_username: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub notification_type: String,
    pub payload: serde_json::Value,
    pub read: bool,
    pub created_at: DateTime<Utc>,
}

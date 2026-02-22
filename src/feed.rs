use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub link: String,
    pub summary: Option<String>,
    pub published: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub title: String,
    pub url: String,
    pub entries: Vec<Entry>,
}

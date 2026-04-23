use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryEntry {
    pub script_name: String,
    pub start_timestamp: u64, // Seconds since Unix epoch
    pub duration_ms: u128,
    pub exit_code: Option<i32>,
}

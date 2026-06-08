use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{KonquerorError, Result};

// --- Operator / Auth ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operator {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub api_key: Option<String>,
    pub is_admin: bool,
    pub logged_in: bool,
}

// --- Listener ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listener {
    pub id: Uuid,
    pub kind: ListenerKind,
    pub aes_key: String,
    pub hmac_key: String,
    pub status: ComponentStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ListenerKind {
    Http,
}

// --- Implant ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Implant {
    pub id: Uuid,
    pub listener_id: Uuid,
    pub os: String,
    pub arch: String,
    pub hostname: String,
    pub username: String,
    pub user_id: String,
    pub pid: u32,
    pub ppid: u32,
    pub cwd: String,
    pub sleep_time_secs: u64,
    pub jitter: u32,
    pub max_retry: u32,
    pub failed_checkins: u32,
    pub kill_date: DateTime<Utc>,
    pub status: ComponentStatus,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

// --- Task ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub implant_id: Uuid,
    pub listener_id: Uuid,
    pub kind: TaskKind,
    pub args: Vec<String>,
    pub result: Option<TaskResult>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub success: bool,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskKind {
    Cmd,
    Powershell,
    Ls,
    Cat,
    Upload,
    Download,
    Ps,
    Ifconfig,
    Set,
    ExecuteAssembly,
    KillImplant,
    KillListener,
}

// --- Shared enums ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComponentStatus {
    Active,
    Inactive,
    Dead,
}

impl TryFrom<&str> for ComponentStatus {
    type Error = KonquerorError;
    
    fn try_from(s: &str) -> Result<Self> {
        match s {
            "Active" => Ok(ComponentStatus::Active),
            "Inactive" => Ok(ComponentStatus::Inactive),
            "Dead" => Ok(ComponentStatus::Dead),
            _ => Err(KonquerorError::InvalidInput(format!("{} is not a valid status", s.to_string())))
        }
    }
}
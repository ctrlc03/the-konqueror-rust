pub mod cmd;
pub mod fs;
pub mod net;
pub mod ps;
pub mod set;

use std::{collections::HashMap, path::Path};

use uuid::Uuid;

pub struct CommandMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
}

pub trait ImplantCommand: Send + Sync {
    fn meta(&self) -> &CommandMeta;
    fn execute(&self, args: &[String], ctx: &mut ImplantContext) -> CommandResult;
}

pub struct CommandResult {
    pub success: bool,
    pub output: String,
}

pub struct ImplantContext {
    pub cwd: String,
    pub os: String,
    pub implant_id: Uuid,
    pub sleep_time_secs: u64,
    pub jitter: u32,
    pub kill_date: chrono::DateTime<chrono::Utc>,
}

impl ImplantContext {
    pub fn resolve_path(&self, path: &str) -> String {
        if Path::new(path).is_absolute() {
            path.to_string()
        } else {
            format!("{}/{}", &self.cwd, &path)
        }
    }
}

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn ImplantCommand>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, cmd: Box<dyn ImplantCommand>) {
        self.commands.insert(cmd.meta().name.to_string(), cmd);
    }

    pub fn get(&self, name: &str) -> Option<&dyn ImplantCommand> {
        self.commands.get(name).map(|cmd| cmd.as_ref())
    }

    pub fn list(&self) -> Vec<&CommandMeta> {
        self.commands.values().map(|cmd| cmd.meta()).collect()
    }
}

#[cfg(test)]
pub(crate) fn test_context() -> ImplantContext {
    use chrono::Utc;

    ImplantContext {
        cwd: std::env::temp_dir().to_string_lossy().to_string(),
        os: std::env::consts::OS.to_string(),
        implant_id: uuid::Uuid::new_v4(),
        sleep_time_secs: 0u64,
        jitter: 0u32,
        kill_date: Utc::now(),
    }
}

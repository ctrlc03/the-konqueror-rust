pub mod cmd;
pub mod fs;

use std::{collections::HashMap, path::Path};

use uuid::Uuid;

pub struct CommandMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
}

pub trait ImplantComand: Send + Sync {
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
    commands: HashMap<String, Box<dyn ImplantComand>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, cmd: Box<dyn ImplantComand>) {
        self.commands.insert(cmd.meta().name.to_string(), cmd);
    }

    pub fn get(&self, name: &str) -> Option<&dyn ImplantComand> {
        self.commands.get(name).map(|cmd| cmd.as_ref())
    }

    pub fn list(&self) -> Vec<&CommandMeta> {
        self.commands.values().map(|cmd| cmd.meta()).collect()
    }
}

#[cfg(test)]
pub(crate) fn test_context() -> ImplantContext {
    ImplantContext {
        cwd: std::env::temp_dir().to_string_lossy().to_string(),
        os: std::env::consts::OS.to_string(),
        implant_id: uuid::Uuid::new_v4(),
    }
}

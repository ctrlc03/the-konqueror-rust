use crate::commands::{CommandRegistry, cmd::CmdCommand};

pub mod commands;

fn main() {
    let mut cmd_registry = CommandRegistry::new();

    cmd_registry.register(Box::new(CmdCommand));
}

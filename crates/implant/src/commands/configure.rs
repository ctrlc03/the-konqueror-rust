use crate::commands::{CommandMeta, CommandResult, ImplantCommand};

pub struct ConfigureCmd;

impl ConfigureCmd {
    const META: CommandMeta = CommandMeta {
        name: "set",
        description: "Set a config",
        usage: "set <config> <value>",
    };
}

impl ImplantCommand for ConfigureCmd {
    fn meta(&self) -> &CommandMeta {
        &Self::META
    }

    fn execute(&self, _: &[String], _: &mut super::ImplantContext) -> CommandResult {
        
        CommandResult {
            success: true,
            output: "".to_string(),
        }
    }
}

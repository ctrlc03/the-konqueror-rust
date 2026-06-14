use sysinfo::System;

use crate::commands::{CommandMeta, CommandResult, ImplantCommand, ImplantContext};

pub struct PsCommand;

impl PsCommand {
    const META: CommandMeta = CommandMeta {
        name: "ps",
        description: "List running processes",
        usage: "ps",
    };
}

impl ImplantCommand for PsCommand {
    fn meta(&self) -> &CommandMeta {
        &Self::META
    }

    fn execute(&self, _: &[String], _: &mut ImplantContext) -> CommandResult {
        let mut sys = System::new_all();

        let mut output = String::new();

        // First we update all information of our `System` struct.
        sys.refresh_all();

        for (pid, process) in sys.processes() {
            output += format!("[{pid}] {:?} {:?}\n", process.name(), process.disk_usage()).as_ref();
        }

        CommandResult {
            success: true,
            output,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::test_context;

    #[test]
    fn test_get_processes() {
        let ps = PsCommand;
        let mut ctx = test_context();

        let result = ps.execute(&[], &mut ctx);
        assert_eq!(result.success, true);
        assert!(!result.output.is_empty());
    }
}

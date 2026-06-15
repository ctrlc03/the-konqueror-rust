use crate::commands::{CommandMeta, CommandResult, ImplantCommand, ImplantContext};
use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;

pub struct IfConfigCommand;

impl IfConfigCommand {
    const META: CommandMeta = CommandMeta {
        name: "ifconfig",
        description: "List network interfaces",
        usage: "ifconfig",
    };
}

impl ImplantCommand for IfConfigCommand {
    fn meta(&self) -> &CommandMeta {
        &Self::META
    }

    fn execute(&self, _: &[String], _: &mut ImplantContext) -> CommandResult {
        let Ok(network_interfaces) = NetworkInterface::show() else {
            return CommandResult {
                success: false,
                output: "could not get network interfaces".to_string(),
            };
        };

        let mut output = String::new();

        for itf in network_interfaces.iter() {
            output += format!("{:?}", itf).as_ref();
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
    fn test_get_interfaces() {
        let cmd = IfConfigCommand;
        let mut ctx = test_context();
        let result = cmd.execute(&[], &mut ctx);

        assert_eq!(result.success, true);
        assert!(!result.output.is_empty());
    }
}

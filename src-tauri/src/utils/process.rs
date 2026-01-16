use log::error;
use std::{
    path::Path,
    process::{Command, Stdio},
};

use crate::types::error::CustomError;

pub fn run_command<P: AsRef<std::ffi::OsStr>>(
    program: P,
    args: Option<&[&str]>,
    envs: Option<&[(&str, &str)]>,
    cwd: Option<&Path>,
) -> Result<(), CustomError> {
    let mut cmd = Command::new(&program);

    if let Some(arguments) = args {
        if !arguments.is_empty() && !(arguments.len() == 1 && arguments[0].is_empty()) {
            cmd.args(arguments);
        }
    }

    if let Some(env_list) = envs {
        cmd.envs(env_list.iter().cloned());
    }

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    //suppressing stdout & stderr terminal logs
    cmd.stdout(Stdio::null()).stderr(Stdio::null());

    // LOGGING THE COMMAND
    println!(
        "Running command: {:?} {}",
        program.as_ref(),
        args.map_or(String::new(), |a| a.join(" "))
    );

    if let Some(env_list) = envs {
        println!("With envs:");
        for (key, val) in env_list {
            println!("  {}={}", key, val);
        }
    }

    let status = cmd.status()?;

    if !status.success() {
        error!(
            "Command {:?} failed with status: {:?}",
            program.as_ref(),
            status
        );
        return Err(CustomError::Anyhow(anyhow::anyhow!(
            "Command {:?} failed with status: {:?}",
            program.as_ref(),
            status
        )));
    }

    Ok(())
}

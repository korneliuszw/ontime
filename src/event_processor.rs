use crate::config;
use crate::errors::ExecutionError;
use crate::event::{Event, ExecutionType};
use crate::CONFIG;
use std::fs::OpenOptions;
use std::io::stderr;
use std::os::unix::io::AsRawFd;
// According to docs Child::output.status() requires this trait to import to return signals as well
#[allow(unused_imports)]
use std::os::unix::process::ExitStatusExt;
use std::{
    os::unix::prelude::FromRawFd,
    process::{Command, Stdio},
};
pub fn process_event(
    event: &mut Event,
    execution_type: &ExecutionType,
) -> Result<(), Box<dyn std::error::Error>> {
    match execution_type {
        ExecutionType::START => {
            event.executed = (true, event.executed.1);
            let result = execute(event.execute_start.as_str());
            if let Err(x) = &result {
                if x.is::<ExecutionError>() && (CONFIG.fail & config::FAIL_RETRY) > 0
                {
                    debug!("OK");
                    event.executed = (false, event.executed.1);
                }
            }
            return result;
        }
        ExecutionType::END => {
            event.executed = (event.executed.0, true);
            let result = execute(event.execute_end.as_str());
            if let Err(x) = &result {
                if x.is::<ExecutionError>() && (CONFIG.fail & config::FAIL_RETRY) > 0 {
                    debug!("OK");
                    event.executed = (event.executed.0, false);
                }
            }
            return result;
        }
        ExecutionType::LOOP => {
            if let Some(during) = &event.during {
                return execute(during.as_str());
            }
            return Ok(());
        }
        _ => return Ok(()),
    };
}
pub fn execute(what: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut split = what.split(" ");
    let mut builder = Command::new(split.next().unwrap());
    builder.args(split);
    if CONFIG.pipe > config::PIPE_FROM_NONE {
        if (CONFIG.pipe & config::PIPE_FROM_STDERR) > 0 {
            builder.stderr(pipe_to()?);
        }
        if (CONFIG.pipe & config::PIPE_FROM_STDOUT) > 0 {
            builder.stdout(pipe_to()?);
        }
    }
    let output = builder.spawn()?.wait_with_output()?;
    let code = output.status.code().unwrap();
    if code > CONFIG.fail_on_code {
        error!("Execution failed with code {}", code);
        // TODO: Implement behaviour
        let err = ExecutionError {};
        return Err(err.into());
    }
    Ok(())
}
fn pipe_to() -> Result<Stdio, Box<dyn std::error::Error>> {
    match &CONFIG.pipe_to {
        &config::PipeTo::FILE => Ok(OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(CONFIG.file.as_ref().unwrap())?
            .into()),
        &config::PipeTo::STDERR => unsafe { Ok(Stdio::from_raw_fd(stderr().as_raw_fd())) },
        &config::PipeTo::STDOUT => Ok(Stdio::inherit()),
        &config::PipeTo::NONE => Ok(Stdio::null()),
    }
}

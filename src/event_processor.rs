use crate::config;
use crate::event::{Event, ExecutionType};
use crate::CONFIG;
use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io::{stderr, stdout};
use std::os::unix::io::AsRawFd;
use std::rc::Rc;
use std::{
    os::unix::prelude::FromRawFd,
    process::{Command, Stdio},
};
pub fn process_event(
    event: Rc<RefCell<Event>>,
    execution_type: &ExecutionType,
) -> Result<(), Box<dyn std::error::Error>> {
    // Smaller lifetime to make sure borrowed reference gets dropped so we can write afterwards
    {
        let mut event = event.borrow_mut();
        match execution_type {
            ExecutionType::START => {
                event.executed = (true, event.executed.1);
                return execute(event.execute_start.as_str());
            }
            ExecutionType::END => {
                event.executed = (event.executed.0, true);
                return execute(event.execute_end.as_str());
            }
            _ => return Ok(()),
        };
    }
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

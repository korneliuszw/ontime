use std::path::PathBuf;

use chrono::Duration;
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Korneliusz W.")]
pub struct Config {
    #[clap(short, long, about="Maximum time distance between current time and unexecuted event, in which the pending event will be executed (in minutes)", default_value = "90")]
    pub distance: i64,
    #[clap(long, about="Maximum time distance between current time and unexecuted start event, in which the pending event will be executed (in minutes)")]
    pub distance_start: Option<i64>,
    #[clap(long, about="Maximum time distance between current time and unexecuted end event, in which the pending event will be executed (in minutes)")]
    pub distance_end: Option<i64>,
    #[clap(
        short,
        long,
        default_value = "0",
        about = "Flags specyfing behaviour when execution fails, 0 - nothing changes, 1 - retry on a next check, 2 - don't write to the cache"
    )]
    pub fail: u32,
    #[clap(
        long,
        default_value = "1",
        about = "Every return code greater than or equal this argument will be considered failed"
    )]
    pub fail_on_code: u32,
    #[clap(
        short,
        long,
        default_value = "1",
        about = "Flags specyfing what to pipe: 1 - stderr 2 - stdout"
    )]
    pub pipe: u32,
    #[clap(
        long,
        default_value = "stderr",
        about = "Pipes to \"stderr\", \"stdout\", \"none\" or \"file\" (requires --file argument)"
    )]
    pub pipe_to: PipeTo,
    #[clap(
        long,
        about = "Pipes to the given file, only works when --pipe-to is set to a file"
    )]
    pub file: Option<PathBuf>,
}
pub const FAIL_REGULAR : u32 = 0x0;
pub const FAIL_RETRY : u32 = 0x1;
pub const FAIL_DO_NOT_WRITE : u32 = 0x2;
pub const PIPE_FROM_NONE : u32 = 0x0;
pub const PIPE_FROM_STDOUT : u32 = 0x1;
pub const PIPE_FROM_STDERR : u32 = 0x2;

#[derive(Debug)]
pub enum PipeTo {
    STDOUT,
    STDERR,
    FILE,
    NONE,
}
impl std::str::FromStr for PipeTo {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "file" => Ok(Self::FILE),
            "stderr" => Ok(Self::STDERR),
            "stdout" => Ok(Self::STDOUT),
            "none" => Ok(Self::NONE),
            _ => Err("Value not allowed".into()),
        }
    }
}
fn time_to_duration(time: &str) -> Result<Duration, String> {
    let time = match time.parse::<i64>() {
        Ok(x) => x,
        Err(_) => return Err("Argument for parameter distance must be an integer".into()),
    };
    Ok(Duration::minutes(time))
}

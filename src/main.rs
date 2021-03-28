extern crate pretty_env_logger;
extern crate yaml_rust;
#[macro_use]
extern crate log;
extern crate nix;
#[macro_use]
extern crate lazy_static;
extern crate clap;

mod config;
mod directory;
mod errors;
mod event;
mod event_cache;
mod event_list;
mod event_processor;
mod plan;
mod weekday;

use crate::directory::{filter_dir_content, read_env_dir_or_fallback_to_etc};
use crate::event::ExecutionType;
use crate::event_processor::process_event;
use chrono::{offset::Local, Datelike};
use clap::Clap;
use std::path;
use std::rc::Rc;
use std::time::Duration;
use weekday::chrono_to_string;

const APP_NAME: &'static str = "ontime";

lazy_static! {
    static ref CONFIG: config::Config = config::Config::parse();
}

fn main() {
    pretty_env_logger::init();
    // Initialize CONFIG early to parse the config
    {
        &CONFIG.distance;
    }
    match real_main() {
        Err(err) => {
            error!("{:?}", err);
        }
        _ => {
            error!("Weird, this shouldn't end");
        }
    }
    std::process::exit(1);
}
fn real_main() -> Result<(), Box<dyn std::error::Error>> {
    let dir_content = read_env_dir_or_fallback_to_etc(
        "XDG_CONFIG_HOME",
        APP_NAME,
        true,
        Some(filter_dir_content),
    )?;
    time_loop(&dir_content)?;
    return Ok(());
}
fn time_loop(conf_files: &Vec<path::PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let mut plan = plan::get_plan(&Local::now(), conf_files)?;
    loop {
        let now = Local::now();
        if plan.weekday != chrono_to_string(&now.weekday()) {
            plan = plan::get_plan(&now, conf_files)?;
        }
        if plan.events.head.is_none() {
            std::thread::sleep(Duration::from_secs(60));
        }
        let mut current_link = plan.events.head.as_mut();
        let mut changed = false;
        // Iterate until there are no more elements (links) in the list
        while let Some(current) = current_link {
            let execution = current.value.borrow().should_execute(&now.timestamp());
            match execution {
                ExecutionType::NONE => {}
                _ => {
                    changed = true;
                    process_event(
                        Rc::clone(&current.value),
                        &execution,
                    )?
                }
            }
            current_link = current.next_mut();
        }
        if changed {
            plan.cache.write(&plan.events)?;
        }
        std::thread::sleep(Duration::from_secs(60));
    }
}

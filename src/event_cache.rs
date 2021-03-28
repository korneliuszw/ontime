use crate::directory::find_env_dir_or_etc;
use crate::event::Event;
use crate::event_list::List;
use crate::APP_NAME;
use chrono::NaiveDate;
use std::cell::RefCell;
use std::fs;
use std::io::{self, BufRead, Seek, Write};
use std::rc::Rc;
const CACHE_FILE_NAME: &'static str = "ontime.cache";

/// Struct holding variables responsible for reading through cache
/// Cache contains execution status of today's events
/// Header (first line) should contain date
/// Then every event line is going to be in form of checksum and two 0 or 1 (first for start, second for end script) meaning whether execution happend

#[derive(Debug)]
pub struct Cache {
    date: NaiveDate,
    // TODO: Replace with actual descriptor type??
    descriptor: fs::File,
}
impl Cache {
    /// Tries to read cache, firstly in $XDG_CONFIG_HOME/ontime directory, then in /etc/ontime
    /// A File where the cache is storied is named ontime.cache
    /// If file doesn't have a header (file is empty) it sets date to 0000-00-00 so cleanup should be always called, right after initialization
    pub fn initial_read_cache() -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = find_env_dir_or_etc("XDG_CACHE_HOME", "")?;
        path.push(CACHE_FILE_NAME);
        let descriptor = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .create(true)
            .open(path)?;
        let reader = io::BufReader::new(&descriptor);
        let mut lines = reader.lines();
        let date = match lines.next() {
            None => NaiveDate::from_ymd(1990, 1, 1),
            Some(x) => NaiveDate::parse_from_str(&x?, "%Y-%m-%d")?,
        };
        Ok(Self { date, descriptor })
    }
    /// Checks whether the date of the cache matches the current date
    /// If not, empties the file and writes the current date.
    pub fn cleanup(&mut self, time_now: &NaiveDate) -> io::Result<()> {
        if &self.date == time_now {
            return Ok(());
        }
        debug!("Cleaning cache");
        self.date = time_now.to_owned();
        self.descriptor.set_len(0)?;
        self.descriptor.seek(io::SeekFrom::Start(0))?;
        self.descriptor.write_all(self.date.to_string().as_bytes())
    }
    pub fn full_read_cache(&mut self, event_list: &mut List<Event>) -> io::Result<()> {
        self.descriptor.seek(io::SeekFrom::Start(0))?;
        let reader = io::BufReader::new(&self.descriptor);
        let mut lines = reader.lines();
        // Skip first line as it contains date only
        match lines.next() {
            None => return Ok(()),
            _ => {}
        }
        while let Some(x) = lines.next() {
            let x = x.unwrap();
            // MD5 checksum (first 32 characters of the line)
            if x.len() == 0 {
                return Ok(())
            }
            let mut split = x.split(" ");
            let checksum = split.next().unwrap();
            let executed_start: bool = match split.next().unwrap().parse::<bool>() {
                Err(_) => false,
                Ok(x) => x,
            };
            let executed_end: bool = match split.next().unwrap().parse::<bool>() {
                Err(_) => false,
                Ok(x) => x,
            };
            let mut current_link = event_list.head.as_mut();
            while let Some(item) = current_link {
                let mut equal = false;
                if let Some(x) = &item.value.borrow().checksum {
                    if x.as_str() == checksum {
                        equal = true;
                    }
                }
                if equal {
                    item.value.borrow_mut().executed = (executed_start, executed_end);
                }
                current_link = item.next_mut();
            }
        }
        Ok(())
    }
    // TODO: Rewrite to print affected event instead of whole file
    pub fn write(&mut self, event_list: &List<Event>) -> io::Result<()> {
        let mut to_write: String = String::new();
        to_write.push_str(&format!("{}\n", self.date.to_string()));
        let mut current_link = event_list.head.as_ref();
        self.descriptor.set_len(0)?;
        self.descriptor.seek(io::SeekFrom::Start(0))?;
        while let Some(item) = current_link {
            let borrow = item.value.borrow();
            if let Some(x) = &borrow.checksum {
                to_write.push_str(&format!(
                    "{} {} {}\n",
                    x, borrow.executed.0, borrow.executed.1
                ));
            }
            current_link = item.next();
        }
        let mut writer = io::LineWriter::new(&self.descriptor);
        writer.write_all(to_write.as_bytes())
    }
}

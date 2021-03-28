use crate::errors::{BadTimeFormat, RequiredAttributeMissingError};
use crate::CONFIG;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::path::PathBuf;
use yaml_rust::Yaml;

#[derive(Debug, Clone, Eq)]
pub struct Event {
    start: i64,
    end: i64,
    // Execute on start
    pub execute_start: String,
    // Execute on end
    pub execute_end: String,
    pub executed: (bool, bool),
    pub checksum: Option<String>,
}
impl Event {
    pub fn new(
        yaml_object: &Yaml,
        weekday: &Cow<'static, str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Event {
            start: parse_time(
                yaml_object["start"]
                    .as_str()
                    .ok_or(RequiredAttributeMissingError::new("start", weekday))?,
                weekday.as_ref(),
            )?,
            end: parse_time(
                yaml_object["end"]
                    .as_str()
                    .ok_or(RequiredAttributeMissingError::new("end", weekday))?,
                weekday.as_ref(),
            )?,
            execute_start: yaml_object["execute_start"]
                .to_owned()
                .into_string()
                .ok_or(RequiredAttributeMissingError::new("execute_start", weekday))?,

            execute_end: yaml_object["execute_end"]
                .to_owned()
                .into_string()
                .ok_or(RequiredAttributeMissingError::new("execute_end", weekday))?,
            executed: (false, false),
            checksum: None,
        })
    }
    /// Calculate MD5 checksum of the event (from start, end, execute_start and execute_end fields)
    /// Sets self.checksum to calculated checksum, in hexadecimal string format
    /// Used to later compare with cache
    pub fn calculate_checksum(&mut self) {
        let self_string = self.to_string();
        self.checksum = Some(format!("{:x}", md5::compute(self_string)));
    }
    /// Compares current time with times of start and end of an event
    /// Returns ExecutionType::NONE if it isn't the right time or execution had been issued previously
    /// Else returns ExecutionType::START or ExecutionType::END depending on the time
    pub fn should_execute(&self, time_now: &i64) -> ExecutionType {
        let distance_start = self.start.to_owned()
            + (CONFIG
                .distance_start
                .or_else(|| Some(CONFIG.distance))
                .unwrap()
                * 60);
        let distance_end = self.end.to_owned()
            + (CONFIG
                .distance_end
                .or_else(|| Some(CONFIG.distance))
                .unwrap()
                * 60);
        if !self.executed.0 && &self.start <= time_now && &distance_start >= time_now {
            debug!("Executing start script {}", self.execute_start);
            return ExecutionType::START;
        } else if self.executed.0
            && !self.executed.1
            && &self.end <= time_now
            && &distance_end >= time_now
        {
            debug!("Executing end script {}", self.execute_end);
            return ExecutionType::END;
        }
        ExecutionType::NONE
    }
}
impl ToString for Event {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}",
            self.start, self.end, self.execute_start, self.execute_end
        )
    }
}
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = other.start.cmp(&self.start);
        ordering
    }
}
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        return self.start == other.start;
    }
}
pub fn parse_time(time: &str, weekday: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let mut split = time.split(':');
    let now = chrono::Local::now();
    let date = NaiveDateTime::new(
        now.date().naive_local(),
        NaiveTime::from_hms(
            split
                .next()
                .ok_or(BadTimeFormat::new(weekday))?
                .parse::<u32>()?,
            split
                .next()
                .ok_or(BadTimeFormat::new(weekday))?
                .parse::<u32>()?,
            0x0,
        ),
    ) - now.offset().to_owned();
    Ok(date.timestamp())
}
#[derive(Debug)]
pub enum ExecutionType {
    START,
    END,
    NONE,
}

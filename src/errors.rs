use std::borrow::Cow;
use std::error;
use std::fmt;
use std::path::PathBuf;
#[derive(Debug, Clone)]

pub struct FileNotFoundError<'a> {
    path: &'a PathBuf,
}

impl fmt::Display for FileNotFoundError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Couldn't read file {}", self.path.to_str().unwrap())
    }
}
impl error::Error for FileNotFoundError<'_> {}

impl<'a> FileNotFoundError<'a> {
    pub fn new(path: &'a PathBuf) -> Self {
        Self { path }
    }
}

#[derive(Debug, Clone)]
pub struct PlanNotFoundError {
    weekday: Cow<'static, str>,
}

impl fmt::Display for PlanNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Plan for {} not found, not doing anything", self.weekday)
    }
}
impl error::Error for PlanNotFoundError {}

impl PlanNotFoundError {
    pub fn new(weekday: Cow<'static, str>) -> Self {
        Self { weekday }
    }
}

#[derive(Debug, Clone)]
pub struct RequiredAttributeMissingError {
    attribute: String,
    weekday: String,
}

impl fmt::Display for RequiredAttributeMissingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Required attribute {} missing for {} plan",
            self.attribute,
            self.weekday.to_string()
        )
    }
}
impl error::Error for RequiredAttributeMissingError {}

impl<'a> RequiredAttributeMissingError {
    pub fn new(attribute: &'a str, weekday: &'a str) -> Self {
        Self {
            weekday: weekday.to_owned(),
            attribute: attribute.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BadTimeFormat {
    weekday: String,
}

impl fmt::Display for BadTimeFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Badly formated time in {}. Make sure it follows HH:MM format",
            self.weekday
        )
    }
}
impl error::Error for BadTimeFormat {}

impl<'a> BadTimeFormat {
    pub fn new(weekday: &'a str) -> Self {
        Self { weekday: weekday.to_owned() }
    }
}
#[derive(Debug, Clone)]
pub struct ExecutionError;

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Script execution failed"
        )
    }
}

impl error::Error for ExecutionError {}



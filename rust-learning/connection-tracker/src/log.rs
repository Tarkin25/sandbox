use std::path::Path;
use std::fs::OpenOptions;
use crate::ConnectionFailure;
use std::fmt::{Display, Result, Formatter};
use chrono::{Duration, Date, Utc, NaiveTime};
use std::io::{Result as IoResult, Write};

pub struct FailureLogger<'a> {
    path: &'a str
}

impl<'a> FailureLogger<'a> {
    pub fn new(path: &'a str) -> IoResult<Self> {
        if ! Path::new(path).exists() {
            Self::init_file(path)?;
        }

        Ok(FailureLogger { path })
    }

    fn init_file(path: &'a str) -> IoResult<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)?;

        writeln!(file, "Start;;Ende;;Dauer")
    }

    pub fn log(&self, failure: &ConnectionFailure) -> IoResult<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(self.path)?;

        let ConnectionFailure { start, end, duration } = failure;

        writeln!(file, "{};{};{};{};{}", Wrapper(start.date()), Wrapper(start.time()), Wrapper(end.date()), Wrapper(end.time()), Wrapper(*duration))
    }
}

struct Wrapper<T>(T);

impl Display for Wrapper<Duration> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}h {}min", self.0.num_hours(), self.0.num_minutes())
    }
}

impl Display for Wrapper<Date<Utc>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0.format("%F"))
    }
}

impl Display for Wrapper<NaiveTime> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0.format("%T"))
    }
}
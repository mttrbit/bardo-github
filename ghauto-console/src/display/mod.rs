use std::fmt::{Display, Formatter, Result};
use chrono::Duration;

pub struct FmtPluralize<'a>(&'a i64, &'a str);

impl<'a> Display for FmtPluralize<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{} {}{}",
            self.0,
            self.1,
            if *self.0 == 1 { "" } else { "s" }
        )
    }
}

pub struct FmtDuration<'a> {
    pub num: i64,
    pub unit: &'a str,
}

impl<'a> Display for FmtDuration<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.num == 0 && self.unit == "minute" {
            return write!(f, "less than a minute ago");
        } else {
            return write!(f, "about {} ago", FmtPluralize(&self.num, &self.unit));
        }
    }
}

impl<'a> FmtDuration<'a> {
    fn new(num: i64, unit: &'a str) -> Self {
        Self {
            num: num,
            unit: unit,
        }
    }

    pub fn fuzzy_ago(ago: Duration) -> Self {
        if ago.num_seconds() < 60 {
            return FmtDuration::new(0, "minute");
        }
        if ago.num_minutes() < 60 {
            let minutes = ago.num_minutes();
            return FmtDuration::new(minutes, "minute");
        }
        if ago.num_hours() < 24 {
            return FmtDuration::new(ago.num_hours(), "hour");
        }
        if ago.num_hours() < 720 {
            return FmtDuration::new(ago.num_hours() / 24, "day");
        }
        if ago.num_hours() < 262800 {
            return FmtDuration::new(ago.num_hours() / 720, "month");
        }

        FmtDuration::new(ago.num_hours() / 262800, "year")
    }
}

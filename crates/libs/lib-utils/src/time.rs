use std::fmt::Display;

use chrono::{Datelike, Local, Timelike, Weekday};

struct Date {
    year: i32,
    month: u32,
    day: u32,
    weekday: Weekday,
}

impl Date {
    fn new(year: i32, month: u32, day: u32, weekday: Weekday) -> Self {
        Self {
            year,
            month,
            day,
            weekday,
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let year = (self.year - 2000).to_string();
        let m = format!("{:02}", self.month);
        let d = format!("{:02}", self.day);
        f.write_str(&format!("{year}{m}{d}"))
    }
}

impl From<(i32, u32, u32, Weekday)> for Date {
    fn from(value: (i32, u32, u32, Weekday)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

struct Time {
    hour: u32,
    _min: u32,
}

impl Time {
    fn new(hour: u32, min: u32) -> Self {
        Self { hour, _min: min }
    }
}

impl From<(u32, u32)> for Time {
    fn from(value: (u32, u32)) -> Self {
        Self::new(value.0, value.1)
    }
}

pub struct Datetime {
    need_session: bool,
    date: Date,
    time: Time,
}

impl Datetime {
    pub fn need_session(mut self) -> Self {
        self.need_session = true;
        self
    }
}

impl From<chrono::DateTime<Local>> for Datetime {
    fn from(value: chrono::DateTime<Local>) -> Self {
        let year = value.year();
        let month = value.month();
        let day = value.day();
        let weekday = value.weekday();

        let date: Date = (year, month, day, weekday).into();

        let time: Time = (value.hour(), value.minute()).into();

        Self {
            date,
            time,
            need_session: false,
        }
    }
}

impl Display for Datetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = &self.date;

        let time = &self.time;
        let session = if self.need_session {
            match (time.hour, date.weekday) {
                (17..=20, Weekday::Wed) => "B",
                (8..=11, _) => "A",
                (13..=16, _) => "B",
                (17..20, _) => "C",
                _ => "",
            }
        } else {
            ""
        };

        let mut title = date.to_string();

        title.push_str(session);

        f.write_str(&title)
    }
}

#[cfg(test)]
mod test {
    use chrono::{Local, TimeZone};

    use super::Datetime;

    #[test]
    fn monday_test() {
        let morning_session =
            Datetime::from(Local.with_ymd_and_hms(2024, 11, 4, 10, 40, 23).unwrap());

        assert_eq!(morning_session.to_string(), "241104A".to_string());

        let afternoon_session =
            Datetime::from(Local.with_ymd_and_hms(2024, 11, 4, 15, 40, 23).unwrap());

        assert_eq!(afternoon_session.to_string(), "241104B".to_string());
        let night_session = Datetime::from(Local.with_ymd_and_hms(2024, 11, 4, 19, 0, 23).unwrap());

        assert_eq!(night_session.to_string(), "241104C".to_string());
    }

    #[test]
    fn tuesday_test() {
        let morning_session =
            Datetime::from(Local.with_ymd_and_hms(2024, 11, 5, 10, 40, 23).unwrap());

        assert_eq!(morning_session.to_string(), "241105A".to_string());

        let afternoon_session =
            Datetime::from(Local.with_ymd_and_hms(2024, 11, 5, 15, 40, 23).unwrap());

        assert_eq!(afternoon_session.to_string(), "241105B".to_string());
        let night_session = Datetime::from(Local.with_ymd_and_hms(2024, 11, 5, 19, 0, 23).unwrap());

        assert_eq!(night_session.to_string(), "241105C".to_string());
    }

    #[test]
    fn wednesday_test() {
        let morning_session =
            Datetime::from(Local.with_ymd_and_hms(2024, 11, 6, 10, 40, 23).unwrap());

        assert_eq!(morning_session.to_string(), "241106A".to_string());

        let night_session = Datetime::from(Local.with_ymd_and_hms(2024, 11, 6, 19, 0, 23).unwrap());

        assert_eq!(night_session.to_string(), "241106B".to_string());
    }

    #[test]
    fn thursday_test() {
        let morning_session =
            Datetime::from(Local.with_ymd_and_hms(2024, 11, 7, 10, 40, 23).unwrap());

        assert_eq!(morning_session.to_string(), "241107A".to_string());
    }
}

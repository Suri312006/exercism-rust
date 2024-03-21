use core::fmt;
use std::{fmt::Formatter, future::pending, ops::Add};

// this is the data structure / object
pub struct Clock {
    hours: i32,
    minutes: i32,
}

// the methods on the clock object
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (final_minutes, hour_overflow) = parse_minutes_hour_overflow(&minutes);
        let total_hours = hours + hour_overflow;

        let final_hours = parse_hour(&total_hours);

        return Clock {
            hours: final_hours,
            minutes: final_minutes,
        };
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // we could be given any amount of minutes
        return Clock::new(self.hours, self.minutes + minutes);
    }
}

fn parse_hour(hours: &i32) -> i32 {
    let hours = hours.clone();

    if hours < 0 {
        return 24 - hours.abs() % 24;
    }
    return hours % 24;
}

fn parse_minutes_hour_overflow(minutes: &i32) -> (i32, i32) {
    let mut minutes = minutes.clone();

    // getting rid of the days
    minutes = minutes % (60 * 24);

    // hours to add to the hours
    let add_hours = minutes / 60;

    // should be the final minutes value
    minutes = minutes % 60;

    if minutes < 0{
        return(60-minutes.abs(), add_hours -1);
    }

    return (minutes, add_hours);
}

//NOTE: prints out clock?
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hours = String::from("");
        let mut minutes = String::from("");

        if self.hours <= 9 {
            hours.push('0');
            let y = char::from_digit(self.hours as u32, 10).unwrap_or_else(|| 'x');
            hours.push(y);
        } else {
            hours = hours.add(&self.hours.to_string());
        }

        if self.minutes <= 9 {
            minutes.push('0');
            let x = char::from_digit(self.minutes as u32, 10).unwrap_or_else(|| 'x');
            minutes.push(x);
        } else {
            // what is actually happening here
            minutes = minutes.add(&self.minutes.to_string());
        }

        return write!(f, "{}:{}", hours, minutes);
    }
}

//NOTE: formatter for clock?
impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return f
            .debug_struct("Clock")
            .field("hours", &self.hours)
            .field("minutes", &self.minutes)
            .finish();
    }
}

//NOTE: equality trait
impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

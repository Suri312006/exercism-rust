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
        let mut start_minutes = minutes.clone();
        let mut start_hours = hours.clone();

        // getting rid of the days
        start_minutes = start_minutes % (60 * 24);

        // hours to add to the hours
        let add_hours = start_minutes / 60;
        start_hours += add_hours;

        // should be the final minutes value
        start_minutes = start_minutes % 60;

        // now to start working with the hours

        // removes all "days in hours"
        start_hours = start_hours % 24;

        return Clock {
            hours: start_hours,
            minutes: start_minutes,
        };
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // we could be given any amount of minutes
        return Clock::new(self.hours, self.minutes + minutes);
    }
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

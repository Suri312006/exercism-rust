use core::fmt;
use std::fmt::Formatter;

// this is the data structure / object
pub struct Clock {
    hours: i32,
    minutes: i32,
}

// the methods on the clock object
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        return Clock { hours, minutes };
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        if self.minutes + minutes >= 60 {
            return Clock::new(self.hours + 1, self.minutes + minutes - 60);
        }
        return Clock::new(self.hours, self.minutes + minutes);

        //        todo!("Add {minutes} minutes to existing Clock time");
    }
}

//NOTE: prints out clock?
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "hours: {} minutes {}", self.hours, self.minutes);
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

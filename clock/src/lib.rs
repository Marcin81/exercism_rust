use std::fmt;
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    const HOURS: i32 = 24;
    const MINUTES: i32 = 60;

    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * Self::MINUTES) + minutes;

        Clock {
            hours: total_minutes.div_euclid(Self::MINUTES).rem_euclid(Self::HOURS),
            minutes: total_minutes.rem_euclid(Self::MINUTES)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

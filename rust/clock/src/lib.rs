use std::fmt;
#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    fn fix_overflow(&self) -> Self {
        let new_hours = (self.hours + (self.minutes / 60)) % 24;
        let new_minutes = self.minutes % 60;
        Clock { hours: new_hours, minutes: new_minutes }
    }
    fn fix_negative(&self) -> Self {
        if self.minutes < 0 || self.hours < 0 {
            let total_minutes = self.minutes + self.hours * 60;
            let total_minutes_positive = 60 * 24 + total_minutes % (60 * 24);
            Clock { hours: 0, minutes: total_minutes_positive }
        } else { Clock { hours: self.hours, minutes: self.minutes } }
    }
    fn fix_all(&self) -> Self {
        self.fix_negative().fix_overflow()
    }
    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_minutes: i32 = minutes % 60;
        let new_hours: i32 = (hours + minutes / 60) % 24;
        Clock { hours: new_hours, minutes: new_minutes }.fix_all()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        (Clock { hours: self.hours, minutes: self.minutes + minutes }).fix_all()
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        (self.minutes == other.minutes) && (self.hours == other.hours)
    }
}
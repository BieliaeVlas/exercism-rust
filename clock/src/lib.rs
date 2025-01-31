
#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}
const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = MINUTES_PER_HOUR * HOURS_PER_DAY;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let add_min = Self::total_minutes(hours, minutes);
        let minutes = add_min % MINUTES_PER_HOUR;
        let hours = add_min / MINUTES_PER_HOUR;
        Self {
            hours,
            minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
    fn total_minutes(hours: i32, minutes: i32) -> i32 {
        let total_minutes = (hours * MINUTES_PER_HOUR + minutes) % MINUTES_PER_DAY;
        if total_minutes >= 0 {
            total_minutes
        } else {
            total_minutes + MINUTES_PER_DAY
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

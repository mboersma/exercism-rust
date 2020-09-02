/// Implements a clock that handles times without dates.
///
/// # Examples
///
/// ```
/// let clock = clock::Clock::new(0, 45).add_minutes(35);
/// assert_eq!(clock.to_string(), "01:20");
/// ```
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut c = Self { hours, minutes };
        c.update();
        c
    }

    fn update(&mut self) {
        self.hours += self.minutes / 60;
        self.minutes %= 60;
        self.hours %= 24;
        if self.minutes < 0 {
            self.minutes += 60;
            self.hours -= 1;
        }
        if self.hours < 0 {
            self.hours += 24;
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut c = Self {
            hours: self.hours,
            minutes: self.minutes + minutes,
        };
        c.update();
        c
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

//! Manages random and unique robot names.

use rand::Rng;

// A Robot has factory settings: specifically a unique, random name.
pub struct Robot {
    pub name: String,
}

/// Manages a robot and its unique, random name.
///
/// # Examples
///
/// ```
/// let robot = robot_name::Robot::new();
/// assert_eq!(robot.name.len(), 5);
/// assert!(robot.name[..2].chars().all(|c| c.is_alphabetic()));
/// assert!(robot.name[2..].chars().all(|c| c.is_numeric()));
/// ```
impl Robot {
    // Creates a new Robot with a unique, random name.
    pub fn new() -> Self {
        Robot {
            name: Self::gen_name(),
        }
    }

    // Generates a unique, random name string consisting of two letters and
    // three digits.
    fn gen_name() -> String {
        let mut rng = rand::thread_rng();
        let mut name = String::new();
        for _ in 0..2 {
            name.push(rng.gen_range(b'A', b'Z') as char);
        }
        for _ in 0..3 {
            name.push(rng.gen_range(b'0', b'9') as char);
        }
        name
    }

    // Returns the unique, random name of this Robot, such as  "RX837" or "BC811".
    pub fn name(&self) -> &str {
        &self.name
    }

    // Returns a Robot to its factory settings, giving it a new name.
    pub fn reset_name(&mut self) {
        self.name = Self::gen_name();
    }
}

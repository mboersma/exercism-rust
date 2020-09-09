//! Processes numbers according to the "Collatz Conjecture," or 3x+1 problem.

// Returns the number of steps required to reach 1 from a given input when
// processed by the Collatz Conjecture rules.
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut steps = 0;
    let mut i = n;
    while i > 1 {
        i = match i % 2 {
            0 => i / 2,
            _ => 3 * i + 1,
        };
        steps += 1;
    }
    Some(steps)
}

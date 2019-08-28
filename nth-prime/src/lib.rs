struct PrimeCandidates {
    index: u32,
}

impl PrimeCandidates {
    fn new() -> PrimeCandidates {
        PrimeCandidates { index: 0 }
    }
}

impl Iterator for PrimeCandidates {
    // iterate over the sequence 2, 3, 5, 7, 9, 11...
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 2 {
            self.index += 1;
        } else {
            self.index += 2;
        }
        Some(self.index)
    }
}

pub fn nth(n: u32) -> Option<u32> {
    if n == 0 {
        return None;
    }

    let mut v: Vec<u32> = Vec::new();

    for i in PrimeCandidates::new() {
        for j in PrimeCandidates::new() {
            if j >= i {
                // prime number, since no smaller divisor was ruled out
                v.push(i);
                if v.len() as u32 == n {
                    return Some(i);
                }
                break;
            }
            if i % j == 0 {
                // not a prime since it's even divisble by a smaller number
                break;
            }
        }
    }

    None
}

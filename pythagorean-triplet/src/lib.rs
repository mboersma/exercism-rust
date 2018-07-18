pub fn find() -> Option<u32> {
    // find a pythagorean triplet where a + b + c = 1000
   for a in 1..1000 {
        for b in 1..1000 {
            for c in 1..1000 {
                if a + b + c > 1000 {
                    break;
                }
                if a*a + b*b == c*c && a + b + c == 1000 {
                    return Some(a*b*c);
                }
            }
        }
    }
    None
}

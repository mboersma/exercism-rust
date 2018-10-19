pub fn square_of_sum(n: u32) -> u32 {
    let mut ret: u32 = 0;
    for i in 1..n+1 {
        ret += i
    }
    ret * ret
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut ret: u32 = 0;
    for i in 1..n+1 {
        ret += i * i
    }
    ret
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

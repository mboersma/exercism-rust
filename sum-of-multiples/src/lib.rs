pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|c| factors.iter().any(|f| c % f == 0))
        .fold(0, |acc, c| acc+c)
}

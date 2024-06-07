pub fn egg_count(display_value: u32) -> usize {
    let binary_str = format!("{:b}", display_value);
    binary_str.chars().filter(|&c| c == '1').count()
}

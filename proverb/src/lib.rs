pub fn build_proverb(list: &[&str]) -> String {
    let mut s = String::new();

    if !list.is_empty() {
        for i in 0..list.len() - 1 {
            s += &format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
        }
        s += &format!("And all for the want of a {}.", list[0]);
    }

    s
}

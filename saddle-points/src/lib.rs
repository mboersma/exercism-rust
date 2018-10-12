fn min_in_col(input: &[Vec<u64>], col: usize) -> u64 {
    let mut res = u64::max_value();

    for row in input.iter() {
        for (col_idx, v) in row.iter().enumerate() {
            if col_idx == col && v < &res {
                res = v.clone()
            }
        }
    }

    res
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = vec![];

    for (row_idx, row) in input.iter().enumerate() {
        // find the maximum value in this row
        let max_in_row = *row.iter().max().unwrap_or(&0);

        // find the minimum value in the max value's column
        let col_idx = row.iter().position(|&v| v == max_in_row).unwrap_or(0);
        let min_in_col = min_in_col(input, col_idx);

        // if the values are equal, append the current location
        if max_in_row == min_in_col {
            saddle_points.push((row_idx, col_idx))
        }
    }

    saddle_points
}

//! Recites the lyrics to that old song "99 Bottles of Beer on the Wall."

/// Returns the nth verse of the song.
pub fn verse(n: i32) -> String {
        match n {
                0 => "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n"
                        .to_string(),
                1 => "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n"
                        .to_string(),
                2 => "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.\n"
                        .to_string(),
                _ => format!(
                        "{0} bottles of beer on the wall, {0} bottles of beer.
Take one down and pass it around, {1} bottles of beer on the wall.\n",
                        n.to_string(),
                        (n - 1).to_string()
                ),
        }
}

/// Returns all the verses of the song, separated by newlines.
pub fn sing(start: i32, end: i32) -> String {
        (end..=start)
                .rev()
                .map(|n| verse(n))
                .collect::<Vec<String>>()
                .join("\n")
}

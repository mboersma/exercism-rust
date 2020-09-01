/// Manages a game player's High Score list.
///
/// # Examples
///
/// ```
/// let scores = high_scores::HighScores::new(&[100, 0, 90, 30]);
/// assert_eq!(scores.latest(), Some(30));
/// ```

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_scores = self.scores.to_vec();
        top_scores.sort_unstable();
        top_scores.reverse();
        top_scores.truncate(3);
        top_scores
    }
}

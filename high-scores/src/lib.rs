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
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut scores = self.scores.to_vec();
        scores.sort();
        scores.last().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_vec();
        scores.sort();
        scores.reverse();
        scores.truncate(3);
        scores
    }
}

use std::collections::HashMap;

struct Stats {
    name: String,
    wins: u32,
    losses: u32,
    draws: u32,
}

impl Stats {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }

    pub fn matches_played(&self) -> u32 {
        self.wins + self.losses + self.draws
    }
    pub fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }

    pub fn add_win(&mut self) {
        self.wins += 1;
    }

    pub fn add_loss(&mut self) {
        self.losses += 1;
    }

    pub fn add_draw(&mut self) {
        self.draws += 1;
    }
}

/// Tallies up and summarizes the results of a football competition.
pub fn tally(match_results: &str) -> String {
    let mut competition: HashMap<&str, Stats> = HashMap::new();
    for line in match_results.lines() {
        if line == "" || line.starts_with("#") {
            continue;
        }
        let parts: Vec<&str> = line.split(";").collect();
        assert_eq!(parts.len(), 3);

        if !competition.contains_key(parts[0]) {
            competition.insert(parts[0], Stats::new(parts[0]));
        }
        if !competition.contains_key(parts[1]) {
            competition.insert(parts[1], Stats::new(parts[1]));
        }
        let mut team1 = competition.get(parts[0]).unwrap();
        let mut team2 = competition.get(parts[1]).unwrap();

        let result = parts[2];
        match result {
            "win" => {
                team1.add_win();
                team2.add_loss();
            }
            "loss" => {
                team1.add_loss();
                team2.add_win();
            }
            "draw" => {
                team1.add_draw();
                team2.add_draw();
            }
            _ => (),
        }
    }
    "Team                           | MP |  W |  D |  L |  P".to_string()
}

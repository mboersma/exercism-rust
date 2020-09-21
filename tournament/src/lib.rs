//! Tallies up and summarizes the results of a football competition.

use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Team<'a> {
    name: &'a str,
    wins: u32,
    losses: u32,
    draws: u32,
}

impl<'a> Team<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name: name,
            wins: 0,
            losses: 0,
            draws: 0,
        }
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

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn matches_played(&self) -> u32 {
        self.wins + self.losses + self.draws
    }

    pub fn wins(&self) -> u32 {
        self.wins
    }

    pub fn draws(&self) -> u32 {
        self.draws
    }

    pub fn losses(&self) -> u32 {
        self.losses
    }

    pub fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }
}

/// Tallies up and summarizes the results of a football competition.
pub fn tally(match_results: &str) -> String {
    let mut competition: HashMap<&str, Team> = HashMap::new();

    for line in match_results.lines() {
        if line == "" || line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.split(";").collect();
        assert_eq!(parts.len(), 3);

        if !competition.contains_key(parts[0]) {
            competition.insert(parts[0], Team::new(parts[0]));
        }
        if !competition.contains_key(parts[1]) {
            competition.insert(parts[1], Team::new(parts[1]));
        }

        let result = parts[2];
        let team1 = competition.get_mut(parts[0]).unwrap();
        match result {
            "win" => team1.add_win(),
            "loss" => team1.add_loss(),
            "draw" => team1.add_draw(),
            _ => (),
        }

        let team2 = competition.get_mut(parts[1]).unwrap();
        match result {
            "win" => team2.add_loss(),
            "loss" => team2.add_win(),
            "draw" => team2.add_draw(),
            _ => (),
        }
    }

    let mut s = String::from("Team                           | MP |  W |  D |  L |  P");
    // TODO: sort order!
    competition.retain(|_k, v| {
        s.push_str(&format!(
            "\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            v.name(),
            v.matches_played(),
            v.wins(),
            v.draws(),
            v.losses(),
            v.points()
        ));
        true
    });
    s
}

//! Tallies up and summarizes the results of a football competition.

use std::{cmp::Ordering, collections::HashMap};

/// Records the results of competition for a team.
#[derive(Clone, Copy)]
struct Team<'a> {
    name: &'a str,
    wins: u32,
    draws: u32,
    losses: u32,
}

impl<'a> Team<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    pub fn add_win(&mut self) {
        self.wins += 1;
    }

    pub fn add_draw(&mut self) {
        self.draws += 1;
    }

    pub fn add_loss(&mut self) {
        self.losses += 1;
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn matches_played(&self) -> u32 {
        self.wins + self.draws + self.losses
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
        let parts: Vec<&str> = line.split(";").collect();
        let name1 = parts[0];
        let name2 = parts[1];
        let result = parts[2];

        if !competition.contains_key(name1) {
            competition.insert(name1, Team::new(name1));
        }
        if !competition.contains_key(name2) {
            competition.insert(name2, Team::new(name2));
        }

        let team1 = competition.get_mut(name1).unwrap();
        match result {
            "win" => team1.add_win(),
            "loss" => team1.add_loss(),
            "draw" => team1.add_draw(),
            _ => (),
        }

        let team2 = competition.get_mut(name2).unwrap();
        match result {
            "win" => team2.add_loss(),
            "loss" => team2.add_win(),
            "draw" => team2.add_draw(),
            _ => (),
        }
    }

    let mut s = String::from("Team                           | MP |  W |  D |  L |  P");
    let mut teams: Vec<Team> = competition.iter().map(|(_, team)| team.clone()).collect();
    teams.sort_by(|a, b| {
        let o = b.points().cmp(&a.points());
        match o {
            Ordering::Equal => a.name().cmp(&b.name()),
            _ => o,
        }
    });
    for team in teams {
        s.push_str(&format!(
            "\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team.name(),
            team.matches_played(),
            team.wins(),
            team.draws(),
            team.losses(),
            team.points()
        ));
    }
    s
}

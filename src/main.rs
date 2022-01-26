use std::sync::{Arc, Mutex};

use iterated_prisoners_dilemma::{
    strategy::{AlwaysBetray, AlwaysCooperate, ForeverRevenge, HalfToHalf, Strategy, TitForTat},
    Choice, History,
};

struct Player {
    strategy: Box<dyn Strategy>,
    history: Arc<Mutex<Vec<History>>>,
    archived: Vec<History>,
    score: u64,
}

impl Player {
    fn new(strategy: Box<dyn Strategy>) -> Self {
        Self {
            strategy,
            history: Arc::new(Mutex::new(Vec::new())),
            archived: vec![],
            score: 0,
        }
    }

    pub fn choose(&mut self, history: Arc<Mutex<Vec<History>>>) -> Choice {
        self.strategy.choose(history)
    }

    pub fn choice_history(&self) -> Arc<Mutex<Vec<History>>> {
        self.history.clone()
    }

    pub fn archive_history(&mut self) {
        let mut histories = self.history.lock().unwrap();
        self.archived.append(&mut *histories);
    }

    pub fn archived_history(&self) -> &Vec<History> {
        &self.archived
    }

    pub fn push_history(&mut self, history: History) {
        let mut histories = self.history.lock().unwrap();
        histories.push(history);
    }

    pub fn score(&self) -> u64 {
        self.score
    }

    pub fn add_score(&mut self, score: u64) {
        self.score += score
    }
}

fn show(players: &Vec<Player>, history: bool) {
    if history {
        println!("{:<20}{:<10}{}", "Strategy", "Score", "History",);

        for player in players {
            println!(
                "{:<20}{:<10}{:?}",
                player.strategy.name(),
                player.score(),
                player.archived_history(),
            )
        }
    } else {
        println!("{:<20}{}", "Strategy", "Score",);
        for player in players {
            println!("{:<20}{}", player.strategy.name(), player.score(),)
        }
    }
}

fn round(players: &mut Vec<Player>) {
    let n = players.len();
    for i in 0..n {
        for k in (i..n).rev() {
            let h0 = players[i].choice_history();
            let h1 = players[k].choice_history();
            let s0 = players[i].strategy.name().to_string();
            let s1 = players[k].strategy.name().to_string();
            let c0 = players[i].choose(h1);
            let c1 = players[k].choose(h0);
            players[i].push_history(History::new(k, s1, c1, c0));
            players[k].push_history(History::new(i, s0, c0, c1));
            let (t0, t1) = match (c0, c1) {
                (Choice::Cooperate, Choice::Cooperate) => (3, 3),
                (Choice::Cooperate, Choice::Betray) => (0, 5),
                (Choice::Betray, Choice::Cooperate) => (5, 0),
                (Choice::Betray, Choice::Betray) => (1, 1),
            };
            players[i].add_score(t0);
            players[k].add_score(t1);
        }
    }
}

fn main() {
    let mut players: Vec<Player> = Vec::new();
    players.push(Player::new(Box::new(AlwaysBetray::default())));
    players.push(Player::new(Box::new(AlwaysCooperate::default())));
    players.push(Player::new(Box::new(HalfToHalf::default())));
    players.push(Player::new(Box::new(TitForTat::new(3))));
    players.push(Player::new(Box::new(ForeverRevenge::new(4))));

    // 5 attempts
    for attempt in 0..5 {
        println!("Attempt: {}", attempt);
        // 1000 rounds per attempt
        for r in 0..1000 {
            if r % 100 == 0 {
                println!("Round: {}", r);
            }
            round(&mut players);
        }

        for player in players.iter_mut() {
            player.archive_history();
        }
    }

    show(&players, false);
}

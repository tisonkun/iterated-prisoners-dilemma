use std::sync::{Arc, Mutex};

use crate::{strategy::Strategy, Choice, History};

pub struct TitForTat {
    id: usize,
}

impl TitForTat {
    pub fn new(id: usize) -> Self {
        TitForTat { id }
    }
}

impl Strategy for TitForTat {
    fn choose(&mut self, history: Arc<Mutex<Vec<History>>>) -> Choice {
        let history = history.lock().unwrap();
        let last_choice = history
            .iter()
            .filter(|h| h.opponent_id == self.id)
            .last()
            .map(|h| h.choice);
        last_choice.unwrap_or_else(|| Choice::Cooperate)
    }

    fn name(&self) -> &str {
        "TitForTat"
    }
}

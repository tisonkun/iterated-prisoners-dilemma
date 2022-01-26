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
    fn choose(&mut self, history: Vec<History>) -> Choice {
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

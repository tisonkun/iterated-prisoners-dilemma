use crate::{strategy::Strategy, Choice, History};

pub struct ForeverRevenge {
    id: usize,
}

impl ForeverRevenge {
    pub fn new(id: usize) -> Self {
        ForeverRevenge { id }
    }
}

impl Strategy for ForeverRevenge {
    fn choose(&mut self, history: Vec<History>) -> Choice {
        let ever_betray = history
            .iter()
            .filter(|h| h.opponent_id == self.id)
            .any(|h| h.choice == Choice::Betray);
        if ever_betray {
            Choice::Betray
        } else {
            Choice::Cooperate
        }
    }

    fn name(&self) -> &str {
        "AlwaysRevenge"
    }
}

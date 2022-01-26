// SPDX-License-Identifier: MIT

use std::{cell::RefCell, rc::Rc};

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
    fn choose(&mut self, history: Rc<RefCell<Vec<History>>>) -> Choice {
        let history = history.borrow();
        let ever_betray = history
            .iter()
            .any(|h| h.opponent_id == self.id && h.choice == Choice::Betray);
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

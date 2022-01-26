// SPDX-License-Identifier: MIT

pub mod strategy;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Choice {
    Cooperate,
    Betray,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct History {
    opponent_id: usize,
    opponent_strategy: String,
    opponent_choice: Choice,
    choice: Choice,
}

impl History {
    pub fn new(
        opponent_id: usize,
        opponent_name: String,
        opponent_choice: Choice,
        choice: Choice,
    ) -> Self {
        History {
            opponent_id,
            opponent_strategy: opponent_name,
            opponent_choice,
            choice,
        }
    }

    pub fn choice(&self) -> Choice {
        self.choice
    }
}

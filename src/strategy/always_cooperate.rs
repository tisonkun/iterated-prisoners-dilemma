// SPDX-License-Identifier: MIT

use std::{cell::RefCell, rc::Rc};

use crate::{strategy::Strategy, Choice, History};

#[derive(Default)]
pub struct AlwaysCooperate;

impl Strategy for AlwaysCooperate {
    fn choose(&mut self, _history: Rc<RefCell<Vec<History>>>) -> Choice {
        Choice::Cooperate
    }

    fn name(&self) -> &str {
        "AlwaysCooperate"
    }
}

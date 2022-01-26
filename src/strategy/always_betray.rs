// SPDX-License-Identifier: MIT

use std::{cell::RefCell, rc::Rc};

use crate::{strategy::Strategy, Choice, History};

#[derive(Default)]
pub struct AlwaysBetray;

impl Strategy for AlwaysBetray {
    fn choose(&mut self, _history: Rc<RefCell<Vec<History>>>) -> Choice {
        Choice::Betray
    }

    fn name(&self) -> &str {
        "AlwaysBetray"
    }
}

use std::sync::{Arc, Mutex};

use crate::{strategy::Strategy, Choice, History};

#[derive(Default)]
pub struct AlwaysCooperate;

impl Strategy for AlwaysCooperate {
    fn choose(&mut self, _history: Arc<Mutex<Vec<History>>>) -> Choice {
        Choice::Cooperate
    }

    fn name(&self) -> &str {
        "AlwaysCooperate"
    }
}

use std::sync::{Arc, Mutex};

use crate::{strategy::Strategy, Choice, History};

#[derive(Default)]
pub struct AlwaysBetray;

impl Strategy for AlwaysBetray {
    fn choose(&mut self, _history: Arc<Mutex<Vec<History>>>) -> Choice {
        Choice::Betray
    }

    fn name(&self) -> &str {
        "AlwaysBetray"
    }
}

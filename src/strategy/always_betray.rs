use crate::{strategy::Strategy, Choice, History};

#[derive(Default)]
pub struct AlwaysBetray;

impl Strategy for AlwaysBetray {
    fn choose(&mut self, _history: Vec<History>) -> Choice {
        Choice::Betray
    }

    fn name(&self) -> &str {
        "AlwaysBetray"
    }
}

use crate::{strategy::Strategy, Choice, History};

#[derive(Default)]
pub struct AlwaysCooperate;

impl Strategy for AlwaysCooperate {
    fn choose(&mut self, _history: Vec<History>) -> Choice {
        Choice::Cooperate
    }

    fn name(&self) -> &str {
        "AlwaysCooperate"
    }
}

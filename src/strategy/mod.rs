mod always_betray;
mod always_cooperate;
mod forever_revenge;
mod half_to_half;
mod tit_for_tat;

use std::sync::{Arc, Mutex};

pub use always_betray::AlwaysBetray;
pub use always_cooperate::AlwaysCooperate;
pub use forever_revenge::ForeverRevenge;
pub use half_to_half::HalfToHalf;
pub use tit_for_tat::TitForTat;

use crate::{Choice, History};

pub trait Strategy {
    fn choose(&mut self, history: Arc<Mutex<Vec<History>>>) -> Choice;
    fn name(&self) -> &str;
}

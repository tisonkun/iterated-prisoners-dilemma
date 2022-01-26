use rand::{rngs::ThreadRng, Rng};

use crate::{strategy::Strategy, Choice, History};

pub struct HalfToHalf {
    rng: ThreadRng,
}

impl Default for HalfToHalf {
    fn default() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
}

impl Strategy for HalfToHalf {
    fn choose(&mut self, _history: Vec<History>) -> Choice {
        match self.rng.gen_bool(0.5) {
            true => Choice::Cooperate,
            false => Choice::Betray,
        }
    }

    fn name(&self) -> &str {
        "HalfToHalf"
    }
}

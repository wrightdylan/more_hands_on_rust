use bevy::prelude::{App, Plugin, Resource};
use rand::{
    Rng, SeedableRng,
    distributions::Standard,
    distributions::uniform::{SampleRange, SampleUniform},
    prelude::Distribution,
};
use std::sync::Mutex;

#[cfg(all(not(feature = "pcg"), not(feature = "xorshift")))]
type RngCore = rand::prelude::StdRng;

#[cfg(feature = "pcg")]
type RngCore = rand_pcg::Pcg64Mcg;

#[cfg(feature = "xorshift")]
type RngCore = rand_xorshift::XorShiftRng;

#[derive(Resource)]
pub struct RandomNumberGenerator {
    rng: Mutex<RngCore>,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            rng: Mutex::new(RngCore::from_entropy()),
        }
    }

    pub fn next<T>(&self) -> T
    where Standard: Distribution<T>
    {
        let mut lock = self.rng.lock().unwrap();
        lock.gen()
    }

    pub fn range<T>(&self, range: impl SampleRange<T>) -> T
    where
        T: SampleUniform + PartialOrd,
    {
        let mut lock = self.rng.lock().unwrap();
        lock.gen_range(range)
    }

    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: Mutex::new(RngCore::seed_from_u64(seed)),
        }
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RandomNumberGenerator::new());
    }
}
use bevy::prelude::{App, Plugin, Resource};
use rand::{
    Rng, SeedableRng,
    distributions::Standard,
    distributions::uniform::{SampleRange, SampleUniform},
    prelude::Distribution,
};

#[cfg(all(not(feature = "pcg"), not(feature = "xorshift")))]
type RngCore = rand::prelude::StdRng;

#[cfg(feature = "pcg")]
type RngCore = rand_pcg::Pcg64Mcg;

#[cfg(feature = "xorshift")]
type RngCore = rand_xorshift::XorShiftRng;

/// `RandomNumberGenerator` holds random number generation state, and offers 
/// random number generation services to your program.
/// 
/// `RandomNumberGenerator` defaults to using the 
/// [PCG](https://crates.io/crates/rand_pcg)
/// algorithm. You can specify `xorshift` as a feature flag to use it 
/// instead.
/// 
/// By default, `RandomNumberGenerator` requires mutability---it 
/// is shared in Bevy with `ResMut<RandomNumberGenerator>`. If 
/// you prefer interior mutability (and wish to use 
/// `Res<RandomNumberGenerator>` instead), specify the `locking`
/// feature flag.
/// 
/// ## Example
/// 
/// 
/// ```
/// use my_library::RandomNumberGenerator;
/// let mut my_rng = RandomNumberGenerator::new();
/// let random_number = my_rng.range(1..10);
/// println!("{random_number}");
/// ```
#[derive(Resource)]
pub struct RandomNumberGenerator {
    rng: RngCore,
}

impl RandomNumberGenerator {
    /// Creates a default `RandomNumberGenerator`, with a randomly
    /// selected starting seed.
    pub fn new() -> Self {
        Self {
            rng: RngCore::from_entropy(),
        }
    }

    /// Generates a new random number of the requested type.
    pub fn next<T>(&mut self) -> T
    where Standard: Distribution<T>
    {
        self.rng.gen()
    }

    /// Generates a random number within the specified range.
    /// 
    /// # Arguments
    /// 
    /// * `range` - the range (inclusive or exclusive) within which to 
    /// generate a random number.
    /// 
    /// # Example
    /// 
    /// ```
    /// use my_library::RandomNumberGenerator;
    /// let mut rng = RandomNumberGenerator::new();
    /// let one_to_nine = rng.range(1..10);
    /// let one_to_ten = rng.range(1..=10);
    /// ```
    pub fn range<T>(&mut self, range: impl SampleRange<T>) -> T
    where
        T: SampleUniform + PartialOrd,
    {
        self.rng.gen_range(range)
    }

    /// Creates a new `RandomNumberGenerator`, with a user-specified random seed.
    /// It will produce the same results each time (given the same requests).
    /// 
    /// # Arguments
    /// 
    /// * `seed` - the random seed to use.
    /// 
    /// # Example
    /// 
    /// ```
    /// use my_library::RandomNumberGenerator;
    /// let mut rng1 = RandomNumberGenerator::seeded(1);
    /// let mut rng2 = RandomNumberGenerator::seeded(1);
    /// let results: (u32, u32) = ( rng1.next(), rng2.next() );
    /// assert_eq!(results.0, results.1);
    /// ```
    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: RngCore::seed_from_u64(seed),
        }
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// `Random` is a Bevy plugin that inserts a `RandomNumberGenerator` 
/// resource into your application.
/// 
/// Once you add the plugin (with `App::new().add_plugin(Random)`),
/// you can access a random number generator in systems with
/// `rng: ResMut<RandomNumberGenerator>`.
pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RandomNumberGenerator::new());
    }
}
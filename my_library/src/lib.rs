//! `my_library` provides a suite of helpers to create games with Bevy.
//! 
//! ## What's Included?
//! 
//! `my_library` includes:
//! 
//! * Random number generation facilities.
//! 
//! ## Feature Flags
//! 
//! The following feature flags are supported.
//! 
//! ### Random Number Generation
//! 
//! * The `locking` feature enables interior mutability inside
//! [`RandomNumberGenerator`],
//!   allowing it to be used as a resource (`Res<RandomNumberGenerator>`)
//! rather than requiring mutability (`ResMut<RandomNumberGenerator>`)
//! * You can control which random number generation algorithm is used by
//! specifying *one* of:
//!    * `xorshift` to use the XorShift algorithm.
//!    * `pcg` to use the PCG algorithm.

#[cfg(not(feature = "locking"))]
mod random;
#[cfg(not(feature = "locking"))]
pub use random::*;

#[cfg(feature = "locking")]
mod random_locking;
#[cfg(feature = "locking")]
pub use random_locking::*;

/// [`RandomNumberGenerator`] wraps the `rand` crate. The `rand` crate
/// is re-exported for your convenience.
pub mod rand {
    pub use rand::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_bounds() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.range(1..10);
            assert!(n >= 1);
            assert!(n < 10);
        }
    }

    #[test]
    fn test_reproducibility() {
        let mut rng = (
            RandomNumberGenerator::seeded(1),
            RandomNumberGenerator::seeded(1),
        );
        (0..1000).for_each(|_| {
            assert_eq!(
                rng.0.range(u32::MIN..u32::MAX),
                rng.1.range(u32::MIN..u32::MAX),
            );
        });
    }

    #[test]
    fn test_next_types() {
        let mut rng = RandomNumberGenerator::new();
        let _ : i32 = rng.next();
        let _ = rng.next::<f32>();
    }

    #[test]
    fn test_float() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.range(-5000.0f32..5000.0f32);
            assert!(n.is_finite());
            assert!(n > -5000.0);
            assert!(n < 5000.0);
        }
    }
}

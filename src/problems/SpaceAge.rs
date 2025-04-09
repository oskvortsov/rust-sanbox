// https://exercism.org/tracks/rust/exercises/space-age

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

// topics #[trait, struct]

#[derive(Debug)]
pub struct Duration(f64);

const SECONDS_IN_YEAR: f64 = 365.25 * 24.0 * 60.0 * 60.0;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / SECONDS_IN_YEAR)
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64 = 1.0;

    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::ORBITAL_PERIOD
    }
}

macro_rules! planet {
    ($iden: ident, $arg: expr) => {
        pub struct $iden;
        impl Planet for $iden {
            const ORBITAL_PERIOD: f64 = $arg;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
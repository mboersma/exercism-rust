// space-age: Given an age in seconds, calculate how old
// someone would be on a planet in our Solar System.
#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

const EARTH_YEAR: f64 = 60.0 * 60.0 * 24.0 * 365.25; // sidereal year is 365.256 days

// Note that this function is potentially lossy, due to float conversion.
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            earth_years: (s as f64 / EARTH_YEAR),
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

macro_rules! planet {
    ($n:ident, $p:expr) => {
        pub struct $n;
        impl Planet for $n {
            fn years_during(d: &Duration) -> f64 {
                d.earth_years / $p
            }
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

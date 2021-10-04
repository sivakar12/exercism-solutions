#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    const EARTH_RATIO: f64;
    fn years_during(d: &Duration) -> f64 {
        let earth_age = d.0 as f64 / (365.25 * 24.0 * 60.0 * 60.0);
        let planet_age = earth_age / Self::EARTH_RATIO;
        planet_age
    }
}

macro_rules! planet {
    ($planet_name: ident, $earth_ratio: expr) => {
        pub struct $planet_name;
        impl Planet for $planet_name {
            const EARTH_RATIO: f64 = $earth_ratio;
        }
    };
}

planet!(Mercury, 0.2408467f64);
planet!(Venus, 0.61519726f64);
planet!(Earth, 1.0f64);
planet!(Mars, 1.8808158f64);
planet!(Jupiter, 11.862615f64);
planet!(Saturn, 29.447498f64);
planet!(Uranus, 84.016846f64);
planet!(Neptune, 164.79132f64);

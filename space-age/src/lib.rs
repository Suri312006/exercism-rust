// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    //NOTE: this will be in earth years
    value: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            value: (s as f64) / (31557600.0),
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        const YEARS_PER_EARTH_ORBIT: f64 = 0.2408467;
        d.value / YEARS_PER_EARTH_ORBIT
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        const YEARS_PER_EARTH_ORBIT: f64 = 00.61519726;
        d.value / YEARS_PER_EARTH_ORBIT
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        const YEARS_PER_EARTH_ORBIT: f64 = 1.0;
        d.value / YEARS_PER_EARTH_ORBIT
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        const YEARS_PER_EARTH_ORBIT: f64 = 1.8808158;
        d.value / YEARS_PER_EARTH_ORBIT
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        const YEARS_PER_EARTH_ORBIT: f64 = 11.862615 ;
        d.value / YEARS_PER_EARTH_ORBIT
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        const YEARS_PER_EARTH_ORBIT: f64 = 29.447498;
        d.value / YEARS_PER_EARTH_ORBIT
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        const YEARS_PER_EARTH_ORBIT: f64 = 84.016846;
        d.value / YEARS_PER_EARTH_ORBIT
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        const YEARS_PER_EARTH_ORBIT: f64 = 164.79132;
        d.value / YEARS_PER_EARTH_ORBIT
    }
}

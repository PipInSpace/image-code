// Tools for random number generation

#[allow(unused)]
/// Pseudo-random number generator. Result numbers are `u64` primitives, but are automatically casted to all primitives
///
/// **Warning:
/// Due to the nature of primitive casting from `u64`, casting to `i128, f32, f64` will NEVER return a negative number**
///
/// `s` is the seed/generation number
///
/// `r` is the resulting random number
pub struct RandU64 {
    s: u64,
    r: u64,
}

#[allow(unused)]
impl RandU64 {
    /// Returns new Random generator struct. Takes in an optional seed value. Otherwise, the current time in seconds is used.
    pub fn new(seed: Option<u64>) -> RandU64 {
        match seed {
            Some(seed) => RandU64 { s: seed, r: rand() },
            None => RandU64 {
                s: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                r: rand(),
            },
        }
    }

    /// Increments the generation number s and sets the result field r
    fn increment(&mut self) {
        self.s = self.s.wrapping_add(0xA0761D6478BD642F);
        let t = u128::from(self.s) * u128::from(self.s ^ 0xE7037ED1A0B428DB);
        self.r = (t as u64) ^ (t >> 64) as u64;
    }

    /// Returns a new random number with generic primitive type.
    ///
    /// **Warning:
    /// Due to the nature of primitive casting from `u64`, casting to `i128, f32, f64` will NEVER return a negative number**
    pub fn next(&mut self) -> u64 {
        self.increment();
        self.r as u64
    }
}

pub fn rand() -> u64 {
    let mut s: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    s = s.wrapping_add(0xA0761D6478BD642F);
    let t = u128::from(s) * u128::from(s ^ 0xE7037ED1A0B428DB);
    let r = (t as u64) ^ (t >> 64) as u64;
    r as u64
}

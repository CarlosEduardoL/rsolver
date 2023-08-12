use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Generates a pseudo-random number using the current time as a seed.
///
/// This function uses the current system time since the UNIX_EPOCH to generate a seed for the
/// pseudo-random number generator. The seed is then manipulated using bitwise operations to
/// produce a 16-bit unsigned integer.
pub fn random() -> u16 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_millis(42));
    let seed = now.as_secs() + now.subsec_nanos() as u64;
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    (x & 0xffff) as u16
}

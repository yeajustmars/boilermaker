use rand::Rng;

/// Generates a random i32 between min and max (inclusive).
#[tracing::instrument]
pub fn rand_i32_between(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

/// Generates a random i64 between min and max (inclusive).
#[tracing::instrument]
pub fn rand_i64_between(min: i64, max: i64) -> i64 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

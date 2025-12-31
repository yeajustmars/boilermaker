use std::env;

pub fn is_dev_env() -> bool {
    match env::var("RUST_ENV_IS_DEV") {
        Ok(val) => val == "true",
        Err(_) => false,
    }
}

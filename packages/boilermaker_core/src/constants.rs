use lazy_static::lazy_static;
use regex::Regex;

pub const BRANCH_REGEX: &str = r"^(refs/heads/)?[A-Za-z0-9._/-]+$";
pub const SUBDIR_REGEX: &str = r"^/?[A-Za-z0-9/\-_].*$";

lazy_static! {
    pub static ref BRANCH_PATTERN: Regex = Regex::new(BRANCH_REGEX).unwrap();
    pub static ref SUBDIR_PATTERN: Regex = Regex::new(SUBDIR_REGEX).unwrap();
}

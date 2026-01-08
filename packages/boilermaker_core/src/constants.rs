use lazy_static::lazy_static;
use regex::Regex;

pub const BRANCH_REGEX: &str = r"^(refs/heads/)?[A-Za-z0-9._/-]+$";
pub const SUBDIR_REGEX: &str = r"^/?[A-Za-z0-9/\-_].*$";
pub const TEMPLATE_FILEPATH_VAR_REGEX: &str = r"(?<underscored>___.*?___)|(?<dashed>---.*?---)";

lazy_static! {
    pub static ref BRANCH_PATTERN: Regex = Regex::new(BRANCH_REGEX).unwrap();
    pub static ref SUBDIR_PATTERN: Regex = Regex::new(SUBDIR_REGEX).unwrap();
    pub static ref TEMPLATE_FILEPATH_VAR_PATTERN: Regex =
        Regex::new(TEMPLATE_FILEPATH_VAR_REGEX).unwrap();
}

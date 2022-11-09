pub mod blacklist;
pub mod error;
pub mod fees;
pub mod ownership;
pub mod state;
pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 30;

pub fn is_separator(char: char) -> bool {
    char == '.' || char == '@' || char == '/'
}

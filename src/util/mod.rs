pub use self::distance::ld;
pub use self::stop_words::STOP_WORDS;
pub use self::hash::hash_file;
pub use self::hash::hash;

pub mod string;

mod stop_words;
mod distance;
mod hash;

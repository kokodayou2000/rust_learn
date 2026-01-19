use crate::modules::projects::object_store::path::parts::InvalidPart;

mod parts;


pub const DELIMITER: &str = "/";

pub const DELIMITER_BYTE:u8 = DELIMITER.as_bytes()[0];

pub const DELIMITER_CHAR:char = DELIMITER_BYTE as char;


#[derive(Debug,thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    
    #[error("{}",path)]
    EmptySegment {
        path: String,
    },
    
    #[error("{} {}",path,source)]
    BadSegment {
        path: String,
        source: InvalidPart,
    }

}


#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Path {
    /// The raw path with no leading or trailing delimiters
    raw: String,
}


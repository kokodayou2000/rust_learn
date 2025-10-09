pub const KEY_VAL_HEADER_LEN: u32 = 4;
pub const MERGE_FILE_EXT: &str = "merger";

pub type Result<T> = std::result::Result<T, std::io::Error>;

pub type KeyDir = std::collections::BTreeMap<Vec<u8>, (u64, u32)>;
use std::ops::Range;
use bytes::Bytes;

pub const OBJECT_STORE_COALESCE_DEFAULT: u64 = 1024 * 1024;

pub async fn coalesce_ranges<F, E, Fut>(
    ranges: &[Range<u64>],
    fetch: F,
    coalesce: u64,
) -> Result<Vec<Bytes>, E>
where
    F: Send + FnMut(Range<u64>) -> Fut,
    E: Send,
    Fut: std::future::Future<Output = Result<Bytes, E>> + Send,
{
    let mut coalesced = vec![];
    Ok(coalesced)
}
use std::sync::Arc;
use bytes::Bytes;


#[derive(Debug,Clone)]
pub struct PutPayload(Arc<[Bytes]>);
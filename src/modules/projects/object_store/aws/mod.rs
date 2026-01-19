use core::fmt::{Display, Formatter};
use std::sync::Arc;
use async_trait::async_trait;
use crate::modules::projects::object_store::lib::{Result, ObjectStore, PutOptions, PutResult, PutMultipartOptions};
use crate::modules::projects::object_store::path::Path;
use crate::modules::projects::object_store::payload::PutPayload;
use crate::modules::projects::object_store::upload::MultipartUpload;

#[derive(Debug,Clone)]
pub struct AmazonS3 {
    /// todo
    pub client: Arc<String>,
}

impl Display for AmazonS3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

#[async_trait]
impl ObjectStore for AmazonS3 {
    async fn put_opts(
        &self,
        location: &Path,
        payload: PutPayload,
        opts: PutOptions,
    ) -> Result<PutResult> {
        todo!()
    }

    async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>> {
        todo!()
    }
}

#[derive(Debug)]
struct S3MultiPartUpload {
    part_idx: usize,
    state: Arc<UploadState>
}

#[derive(Debug)]
struct UploadState {
    
}


#[async_trait]
impl MultipartUpload for S3MultiPartUpload {
    
}

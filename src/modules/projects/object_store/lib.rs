use core::fmt::Debug;
use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use crate::modules::projects::object_store::path::Path;
use crate::modules::projects::object_store::payload::PutPayload;
use std::ops::Range;
use futures::stream::BoxStream;
use crate::modules::projects::object_store::path;
use crate::modules::projects::object_store::upload::MultipartUpload;

#[doc(hidden)]
pub type PutMultipartOpts = PutMultipartOptions;


#[async_trait]
pub trait ObjectStore : std::fmt::Display + Send + Sync  + Debug + 'static
{

    async fn put_opts(
        &self,
        location: &Path,
        payload: PutPayload,
        opts: PutOptions,
    ) -> Result<PutResult>;

    async fn put_multipart_opts(
        &self,
        location: &Path,
        opts: PutMultipartOptions,
    ) -> Result<Box<dyn MultipartUpload>>;

    async fn get_ranges(
        &self,
        location: &Path,
        ranges: &[Range<u64>]
    ) -> Result<Vec<Bytes>> {
        unimplemented!("Multipart upload not supported yet")
    }
}


pub trait ObjectStoreExt: ObjectStore {
    fn put(
        &self, location:
        &Path, payload:
        PutPayload
    ) -> impl Future<Output = Result<PutResult>>;

    fn put_multipart(
        &self,
        location: &Path,
    ) -> impl Future<Output = Result<Box<dyn MultipartUpload>>>;

    fn get(
        &self,
        location: &Path
    ) -> impl Future<Output = Result<GetResult>>;

    fn get_range(
        &self,
        location: &Path,
        range: Range<u64>
    ) -> impl Future<Output = Result<Bytes>>;

    fn head(
        &self,
        location: &Path
    ) -> impl Future<Output = Result<ObjectMeta>>;

    fn delete(
        &self,
        location: &Path
    ) -> impl Future<Output = Result<()>>;

    fn copy(
        &self,
        from: &Path,
        to: &Path
    ) -> impl Future<Output = Result<()>>;


    fn copy_if_not_exists(
        &self,
        from: &Path,
        to: &Path,
    ) -> impl Future<Output = Result<()>>;

    fn rename(
        &self,
        from: &Path,
        to: &Path
    ) -> impl Future<Output = Result<()>>;

    fn rename_if_not_exists(
        &self,
        from: &Path,
        to: &Path,
    ) -> impl Future<Output = Result<()>>;

    fn delete_stream(
        &self,
        location: BoxStream<'static, Result<Path>>,
    ) -> BoxStream<'static, Result<Path>>;
}


impl<T> ObjectStoreExt for T
where
    T: ObjectStore + ?Sized ,
{

    async fn put(
        &self,
        location: &Path,
        payload: PutPayload
    ) -> Result<PutResult> {
        unimplemented!("Multipart upload not supported yet")
    }

    async fn put_multipart(
        &self,
        location: &Path,
    ) -> Result<Box<dyn MultipartUpload>> {
        unimplemented!("Multipart upload not supported yet")
    }

    async fn get(
        &self,
        location: &Path
    ) -> Result<GetResult>{
        unimplemented!("Multipart upload not supported yet")
    }

    async fn get_range(
        &self,
        location: &Path,
        range: Range<u64>
    ) -> Result<Bytes> {
        unimplemented!("Multipart upload not supported yet")
    }

    async fn head(
        &self,
        location: &Path
    ) -> Result<ObjectMeta> {
        unimplemented!("Multipart upload not supported yet")
    }

    async fn delete(
        &self,
        location: &Path
    ) -> Result<()> {
        unimplemented!("Multipart upload not supported yet")
    }

    async fn copy(
        &self,
        from: &Path,
        to: &Path
    ) -> Result<()> {
        unimplemented!("Multipart upload not supported yet")
    }

    async fn copy_if_not_exists(
        &self,
        from: &Path,
        to: &Path
    ) -> Result<()> {
        unimplemented!("Multipart upload not supported yet")
    }

    async fn rename(
        &self,
        from: &Path,
        to: &Path
    ) -> Result<()> {
        unimplemented!("Multipart upload not supported yet")
    }

    async fn rename_if_not_exists(
        &self,
        from: &Path,
        to: &Path
    ) -> Result<()> {
        unimplemented!("Multipart upload not supported yet")
    }

    fn delete_stream(
        &self,
        location: BoxStream<'static, Result<Path>>,
    ) -> BoxStream<'static, Result<Path>> {
        unimplemented!("Multipart upload not supported yet")
    }


}



#[derive(Debug,Clone,Default)]
pub struct PutMultipartOptions {

}

#[derive(Debug,Clone,PartialEq,Eq,Default)]
pub struct ObjectMeta {
    /// full path
    pub location: Path,
    /// last modified
    pub last_modified: DateTime<Utc>,
    /// 对象的字节大小
    pub size: u64,
    /// 对象的唯一 编码
    pub e_tag: Option<String>,
    /// 对象的版本标志
    pub version: Option<String>,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;


#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    
    #[error(" {} , {}",store,source)]
    Generic {
        ///
        store: &'static str,
        /// 封装 error
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    #[error(" {} , {}",path,source)]
    NotFound {
        path: String,
        source: Box<(dyn std::error::Error + Send + Sync + 'static)>,
    },

    #[error(" {}",source)]
    InvalidPath {
        #[from]
        source: path::Error,
    },

    #[error("{}",source)]
    JoinError {
        #[from]
        source: tokio::task::JoinError,
    },
    
    #[error("{}",source)]
    NotSupported {
        source: Box<(dyn std::error::Error + Send + Sync + 'static)>,
    },

    #[error("{} {} ",path,source)]
    AlreadyExists {
        path: String,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    #[error("{} {} ",path,source)]
    Precondition {
        path: String,
        source: Box<(dyn std::error::Error + Send + Sync + 'static)>,
    },

    #[error("{} {}",path,source)]
    NotModified {
        path: String,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    
    #[error("{operation} {implementer}")]
    NotImplemented {
        operation: String,
        implementer: String,
    },

    #[error("{path} {source}")]
    PermissionDenied {
        path: String,
        source: Box<(dyn std::error::Error + Send + Sync + 'static)>,
    },
    
    #[error("{path} {source}")]
    Unauthenticated {
        path: String,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    #[error("{key} {store}")]
    UnknownConfigurationKey {
        key: String,
        store: &'static str,
    },

}

/// 让 Error 能转换成 标准 Error
impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        let kind = match &e {
            Error::NotFound { .. } => std::io::ErrorKind::NotFound,
            _ => std::io::ErrorKind::Other,
        };
        Self::new(kind,e)
    }
}


#[derive(Debug,Default,Clone)]
pub struct GetResult {
    // todo
}

#[derive(Debug,Clone,Default)]
pub struct PutOptions {
    // todo
}


#[derive(Debug,Clone,PartialEq,Eq)]
pub struct PutResult {
    // todo
}
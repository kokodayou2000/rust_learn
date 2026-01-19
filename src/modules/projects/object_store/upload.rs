use async_trait::async_trait;


#[async_trait]
pub trait MultipartUpload: Send + std::fmt::Debug {

}


#[async_trait]
impl<W: MultipartUpload + ?Sized> MultipartUpload for Box<W> {

}
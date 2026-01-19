use std::sync::Arc;
use std::task::Poll;
use bytes::Bytes;
use futures::future::BoxFuture;
use crate::modules::projects::object_store::lib::{ObjectMeta, ObjectStore, ObjectStoreExt};
use crate::modules::projects::object_store::path::Path;
use std::task::Context;
use futures::{ready, FutureExt};

/// 默认的 buffer 大小
pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 1024;


/// 一个兼容 tokio IO trait 的异步缓冲读取器
pub struct BufReader {
    /// 数据源
    store: Arc<dyn ObjectStore>,
    /// 对象大小
    size: u64,
    /// 路径
    path: Path,
    /// 对象的当前位置
    cursor: u64,
    /// 单次请求 bytes 大小
    capacity: usize,
    /// buffered 数据
    buffer: Buffer
}

/// 为 BufReader 实现 Debugger
/// 当debugger当时候，能打印出 合适的数据
impl std::fmt::Debug for BufReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufReader")
        .field("size", &self.size)
        .field("path", &self.path)
        .field("capacity", &self.capacity)
        .finish()
    }
}

enum Buffer {
    Empty,
    Pending(BoxFuture<'static, std::io::Result<Bytes>>),
    Ready(Bytes),
}


impl BufReader {

    /// create BufReader
    pub fn new(store: Arc<dyn ObjectStore>, meta: &ObjectMeta) -> Self {
        Self::with_capacity(store, meta, DEFAULT_BUFFER_SIZE)
    }

    /// 创建 BufReader
    pub fn with_capacity(
        store: Arc<dyn ObjectStore>,
        meta: &ObjectMeta,
        capacity: usize,
    ) -> Self {
        Self {
            path: meta.location.clone(),
            size: meta.size as _,
            store,
            capacity,
            cursor: 0,
            buffer: Buffer::Empty,
        }
    }

    fn poll_fill_buf_impl(
        &mut self,
        cx: &mut Context<'_>,
        amnt: usize,
    ) -> Poll<std::io::Result<&[u8]>> {
        let buf = &mut self.buffer;
        loop {
            match buf {
                Buffer::Empty => {
                    let store = Arc::clone(&self.store);
                    let path = self.path.clone();
                    let start = self.cursor.min(self.size) as _;
                    let end = self.cursor.saturating_add(
                        amnt as u64
                    ).min(self.size) as _;

                    if start == end {
                        return Poll::Ready(Ok(&[]));
                    }

                    *buf = Buffer::Pending(Box::pin(async move {
                        Ok(
                            store.get_range(&path, start..end)
                                .await?
                        )
                    }))

                }
                Buffer::Pending(fut) => match ready!(
                    fut.poll_unpin(cx)
                ) {
                    Ok(b) => {
                        *buf = Buffer::Ready(b);
                    },
                    Err(e) => {
                        return Poll::Ready(Err(e));
                    },
                }
                Buffer::Ready(r) => {
                    return Poll::Ready(Ok(r));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::projects::object_store::aws::AmazonS3;
    use super::*;
    impl BufReader {}
    #[test]
    fn buf_reader_debugger_test(){
        let bufReader = BufReader {
            store: Arc::new(
                AmazonS3{
                    client: Arc::new("".to_string())
                }
            ),
            size: 1,
            path: Path::default(),
            cursor: 1,
            capacity: 1,
            buffer: Buffer::Empty
        };
        println!("bufReader: {:?}", bufReader);
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;
    use tokio::sync::Semaphore;

    #[tokio::test]
    async fn t2() {

        // 启动 5个线程，但是每时每刻只有 3个任务能同时拥有线程
        let semaphore = Arc::new(Semaphore::new(3));
        let mut join_handles = Vec::new();

        for _ in 0..5 {
            // 跨线程
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            join_handles.push(tokio::spawn(async move {
                // perform task...
                // explicitly own `permit` in the task
                // 离开作用域自动 drop
                drop(permit);
            }));
        }

        for handle in join_handles {
            handle.await.unwrap();
        }
    }
}
use async_std::task;
use std::time::Duration;

async fn task() {
    println!("task start sleep");
    task::sleep(Duration::from_secs(1)).await;
    println!("task end sleep");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        task::block_on(task())
    }
}
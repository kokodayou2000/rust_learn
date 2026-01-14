use async_trait::async_trait;
use crate::modules::advance::log::Logger;

#[derive(Debug)]
pub struct FileLogger {
    pub(crate) name: String,
    pub(crate) logs: Vec<String>,
}

#[async_trait]
impl Logger for FileLogger {
    async fn log(&mut self, level: &str, message: &str) -> Result<(), String> {
        // 模拟异步写文件（加一点延迟）
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;

        let line = format!("[FILE {}] {}: {}", self.name, level, message);
        println!("{}", line);
        self.logs.push(line);
        Ok(())
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
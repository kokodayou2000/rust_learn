use core::fmt::Debug;
use async_trait::async_trait;
use crate::modules::advance::log::Logger;

#[derive(Debug)]
pub struct ConsoleLogger {
    pub(crate) name: String,
    pub(crate) logs: Vec<String>,
}

#[async_trait]
impl Logger for ConsoleLogger {
    async fn log(&mut self, level: &str, message: &str) -> Result<(), String> {
        let line = format!("[{}] {}: {}", self.name, level, message);
        println!("{}", line);
        self.logs.push(line);
        Ok(())
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
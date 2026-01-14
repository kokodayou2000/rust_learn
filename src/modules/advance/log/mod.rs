use std::fmt::Debug;

mod console_logger;
mod file_logger;


/// 日志工具 trait
#[async_trait::async_trait]
pub trait Logger: Send + Debug {
    async fn log(&mut self, level: &str, message: &str) -> Result<(), String>;
    fn get_name(&self) -> &str;
}



#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;
    use crate::modules::advance::log::console_logger::ConsoleLogger;
    use crate::modules::advance::log::file_logger::FileLogger;

    fn block_on<F: Future>(f: F) -> F::Output {
        Runtime::new().unwrap().block_on(f)
    }

    #[test]
    fn test_console_logger_via_box() {
        block_on(async {
            let mut logger: Box<ConsoleLogger> = Box::new(ConsoleLogger {
                name: "console-test".to_string(),
                logs: Vec::new(),
            });

            logger.log("INFO", "App started").await.unwrap();
            logger.log("DEBUG", "Debug message here").await.unwrap();
            logger.log("ERROR", "Oops!").await.unwrap();

            assert_eq!(logger.logs.len(),3);
        });
    }

    #[test]
    fn test_file_logger_via_box() {
        block_on(async {
            let mut logger: Box<FileLogger> = Box::new(FileLogger {
                name: "file-test".to_string(),
                logs: Vec::new(),
            });

            logger.log("INFO", "Application started").await.unwrap();
            let name = logger.get_name().to_string();
            logger
                .log("DEBUG", &format!("Logger name is {}", name))
                .await
                .unwrap();
            logger.log("WARN", "Low disk space").await.unwrap();
            assert_eq!(logger.logs.len(),3);
        });
    }

}


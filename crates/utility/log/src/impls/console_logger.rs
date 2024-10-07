use std::collections::HashMap;
use std::fmt::Debug;

use crate::log_level::LogLevel;
use crate::log_trait::Logger;

#[derive(Debug, Clone,Default)]
pub struct ConsoleLogger{
    log_stack: HashMap<LogLevel, Vec<String>>,
    log_file_path: String,
    current_log: String,
}

impl ConsoleLogger {
    pub fn new() -> ConsoleLogger {
        ConsoleLogger {
            log_stack: HashMap::new(),
            log_file_path: "../ConsoleLogText.txt".to_string(),
            current_log: String::new(),
        }
    }
}

impl Logger for ConsoleLogger {
    fn log(&mut self, level: LogLevel, message: &str) {
        let log_message = format!("{:?} : {}", level, message);
        println!("{}", log_message);
        self.log_stack.entry(level).or_insert(Vec::new()).push(log_message.clone());
        self.current_log = log_message;
    }

    fn save_log_file(&self) {
        //ファイルシステムのクレートができてから
    }

    fn get_log(&self, level: LogLevel) -> Vec<String> {
        self.log_stack.get(&level).cloned().unwrap_or_else(Vec::new)
    }

    fn get_current_log(&self, level: Option<LogLevel>) -> String {
        match level {
            Some(level) => {
                let log = self.log_stack.get(&level).unwrap();
                log.last().cloned().unwrap_or_else(String::new)
            }
            None => self.current_log.clone(),
        }
    }

    fn set_log_file_path(&mut self, file_path: &str) {
        self.log_file_path = file_path.to_string();
    }

    fn clear_log(&mut self) {
        self.log_stack.clear();
    }

    fn clear_log_and_save(&mut self) {
        self.save_log_file();
        self.clear_log();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_console_logger() {
        let mut logger = ConsoleLogger::new();
        logger.log(LogLevel::Debug, "debug message");
        logger.log(LogLevel::Error, "error message");
        logger.log(LogLevel::Warning, "warn message");

        let info_logs = logger.get_log(LogLevel::Debug);
        assert_eq!(info_logs.len(), 1);
        assert_eq!(info_logs[0], "Debug : debug message");

        let current_log = logger.get_current_log(None);
        assert_eq!(current_log, "Warning : warn message");

        logger.clear_log();

        let info_logs = logger.get_log(LogLevel::Debug);
        assert_eq!(info_logs.len(), 0);

        logger.log(LogLevel::Debug, "debug message");
    }
}

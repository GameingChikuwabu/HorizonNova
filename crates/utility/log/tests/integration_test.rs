#[cfg(test)]
mod tests {
    use log::*;
    
    #[test]
    fn log_test(){
        log_system::set_log_file_path("test.txt");
        debug_log!("debug message");
        error_log!("error message");
        warn_log!("warn message");

        let info_logs = log_system::get_log(log_level::LogLevel::Warning);

        assert_eq!(info_logs.len(), 1);

        let current_log = log_system::get_current_log(None);

        assert_eq!(current_log, "Warning : [crates\\utility\\log\\tests\\integration_test.rs:10]\n warn message");

        log_system::clear_log();

        let info_logs: Vec<String> = log_system::get_log(log_level::LogLevel::Debug);

        assert_eq!(info_logs.len(),0);

        log_system::save_log_file();
    }
}
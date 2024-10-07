#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        $crate::log_system::log(
            $crate::log_level::LogLevel::Debug,
            &format!("[{}:{}]\n {}", file!(), line!(), format!($($arg)*))
        );
    };
}

#[macro_export]
macro_rules! warn_log {
    ($($arg:tt)*) => {
        $crate::log_system::log(
            $crate::log_level::LogLevel::Warning,
            &format!("[{}:{}]\n {}", file!(), line!(), format!($($arg)*))
        );
    };
}

#[macro_export]
macro_rules! error_log {
    ($($arg:tt)*) => {
        $crate::log_system::log(
            $crate::log_level::LogLevel::Error,
            &format!("[{}:{}]\n {}", file!(), line!(), format!($($arg)*))
        );
    };
}

#[macro_export]
macro_rules! info_log {
    ($($arg:tt)*) => {
        $crate::log_system::log(
            $crate::log_level::LogLevel::Info,
            &format!("[{}:{}]\n {}", file!(), line!(), format!($($arg)*))
        );
    };
}
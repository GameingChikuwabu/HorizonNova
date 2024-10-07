use crate::log_global::*;

/// ログをスタックする
/// 
/// # Arguments
/// 
/// * `level` - ログレベル
/// 
/// * `message` - ログメッセージ
/// 
/// # Examples
/// 
/// ```
/// use log::*;
/// 
/// log_system::log(log_level::LogLevel::Info,"info message");
/// ```
pub fn log(level: crate::log_level::LogLevel, message: &str) {
    let mut logger = GLOBAL_LOGGER.write().unwrap();
    logger.log(level, message);
}

/// ログファイルのパスを設定する
/// 
/// # Arguments
/// 
/// * `file_path` - ログファイルのパス
/// 
/// # Examples
/// 
/// ```
/// use log::*;
/// 
/// log_system::set_log_file_path("test.txt");
pub fn set_log_file_path(file_path: &str) {
    let mut logger = GLOBAL_LOGGER.write().unwrap();
    logger.set_log_file_path(file_path);
}

/// ログファイルを保存する
/// 
/// # Examples
/// 
/// ```
/// use log::*;
/// 
/// log_system::save_log_file();
/// ```
pub fn save_log_file() {
    let logger = GLOBAL_LOGGER.read().unwrap();
    logger.save_log_file();
}

/// ログを取得する
/// 
/// # Arguments
/// 
/// * `level` - ログレベル
/// 
/// # Returns
/// 
/// * `Option<Vec<String>>` - ログ
/// 
/// # Examples
/// 
/// ```
/// use log::*;
/// 
/// let info_logs = log_system::get_log(log_level::LogLevel::Debug);
/// ```
pub fn get_log(level: crate::log_level::LogLevel) -> Vec<String>{
    let logger = GLOBAL_LOGGER.read().unwrap();
    logger.get_log(level)
}

/// 現在のログを取得する
/// 
/// # Arguments
/// 
/// * `level` - ログレベル Noneの場合は全てのログで一番最新のものを取得する
/// 
/// # Returns
/// 
/// * `String` - ログ
/// 
/// # Examples
/// 
/// ```
/// use log::*;
/// 
/// let current_log = log_system::get_current_log(None);
/// ```
pub fn get_current_log(level: Option<crate::log_level::LogLevel>) -> String {
    let logger = GLOBAL_LOGGER.read().unwrap();
    logger.get_current_log(level)
}

/// ログをクリアする
/// 
/// # Examples
/// 
/// ```
/// use log::*;
/// 
/// log_system::clear_log();
/// ```
pub fn clear_log() {
    let mut logger = GLOBAL_LOGGER.write().unwrap();
    logger.clear_log();
}

/// ログをクリアして保存する
/// 
/// # Examples
/// 
/// ```
/// use log::*;
/// 
/// log_system::clear_log_and_save();
/// ```
pub fn clear_log_and_save() {
    let mut logger = GLOBAL_LOGGER.write().unwrap();
    logger.clear_log_and_save();
}

/// グローバルロガーを切り替える
/// 
/// # Arguments
/// 
/// * `logger` - ロガー
/// 
/// # Examples
/// 
/// ```
/// use log::*;
/// 
/// log_system::switching_logger(Box::new(log::impls::console_logger::ConsoleLogger::new()));
/// ```
pub fn switching_logger(logger: Box<dyn crate::log_trait::Logger>) {
    *GLOBAL_LOGGER.write().unwrap() = logger;
}
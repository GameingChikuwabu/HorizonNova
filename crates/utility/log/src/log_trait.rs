use crate::log_level::LogLevel;

pub trait Logger : Send + Sync {
    /// logをスタックする
    /// 
    /// # Arguments
    /// 
    /// * `level` - ログレベル
    /// 
    /// * `message` - ログメッセージ
    fn log(&mut self, level: LogLevel, message: &str);

    /// ログをファイルに保存する
    /// 
    /// # Arguments
    fn save_log_file(&self);

    /// ログを取得する
    /// 
    /// # Arguments
    /// 
    /// * `level` - ログレベル
    fn get_log(&self,level:LogLevel)->Vec<String>;

    /// 現在最新のログを取得する
    /// 
    /// # Arguments
    /// 
    /// * `level` - ログレベル Noneの場合は全てのログで一番最新のものを取得する
    fn get_current_log(&self,level:Option<LogLevel>)->String;

    /// ログファイルのパスを設定する
    /// 
    /// # Arguments
    /// 
    /// * `file_path` - ログファイルのパス
    fn set_log_file_path(&mut self,file_path:&str);

    /// ログをクリアする
    /// 
    fn clear_log(&mut self);

    /// ログをクリアして保存する
    fn clear_log_and_save(&mut self);
}
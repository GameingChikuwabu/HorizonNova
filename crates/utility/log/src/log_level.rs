#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum LogLevel {
    Warning,//警告
    Error,//エラー
    Info,//情報
    Debug,//デバッグ
}
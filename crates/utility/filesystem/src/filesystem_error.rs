use error::{self, Error};

#[derive(Error,Debug)]
pub enum FileSystemError {
    #[error("IO error")]
    IoError(std::io::Error),        // 入出力エラー
    #[error("File not found")]
    FileNotFound,                   // ファイルが見つからない
    #[error("File already exists")]
    FileAlreadyExists,              // ファイルが既に存在している 
    #[error("Permission denied")]
    PermissionDenied,               // 権限が不足している
}
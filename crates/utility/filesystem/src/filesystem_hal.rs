use crate::filesystem_error::FileSystemError;
use crate::filesystem_handler::File;

#[derive(Debug)]
pub enum FileMode{
    Read,    // 読み込み専用
    Write,   // 書き込み専用
    Append,  // 追記モード
}

/// ファイルやディレクトリのメタデータを表す構造体。
#[derive(Debug)]
pub struct FileMetadata {
    pub size: u64,                          // ファイルサイズ
    pub is_dir: bool,                       // ディレクトリかどうか
    pub created: Option<std::time::SystemTime>,  // 作成日時
    pub modified: Option<std::time::SystemTime>, // 更新日時
}

pub trait FileSystemHAL{
    /// 指定されたパスにあるファイルを、指定されたモード（Read, Write, Append）で開きます。
    ///
    /// # 引数
    ///
    /// * `path` - ファイルのパスを表す文字列スライス。
    /// * `mode` - ファイルを開くモード（Read、Write、Append）。
    ///
    /// # 戻り値
    ///
    /// 成功時には `File` オブジェクトを返し、失敗時には `FileError` を返します。
    fn open_file(path: &str, mode: FileMode) -> Result<File, FileSystemError>;

    /// 指定されたファイルからデータを読み取り、バイトベクターとして返します。
    ///
    /// # 引数
    ///
    /// * `file` - 読み取り対象のファイルオブジェクト。
    ///
    /// # 戻り値
    ///
    /// 成功時にはファイルのデータをバイトベクターとして返し、失敗時には `FileError` を返します。
    fn read_file(file: &File) -> Result<Vec<u8>, FileSystemError>;

    /// 指定されたファイルにデータを書き込みます。
    ///
    /// # 引数
    ///
    /// * `file` - 書き込み対象のファイルオブジェクト。
    /// * `data` - 書き込むデータを表すバイトスライス。
    ///
    /// # 戻り値
    ///
    /// 成功時には `()` を返し、失敗時には `FileError` を返します。
    fn write_file(file: &File, data: &[u8]) -> Result<(), FileSystemError>;

    /// 指定されたパスにあるファイルを削除します。
    ///
    /// # 引数
    ///
    /// * `path` - 削除するファイルのパスを表す文字列スライス。
    ///
    /// # 戻り値
    ///
    /// 成功時には `()` を返し、失敗時には `FileError` を返します。
    fn delete_file(path: &str) -> Result<(), FileSystemError>;

    /// 指定されたパスに新しいディレクトリを作成します。
    ///
    /// # 引数
    ///
    /// * `path` - 作成するディレクトリのパスを表す文字列スライス。
    ///
    /// # 戻り値
    ///
    /// 成功時には `()` を返し、失敗時には `FileError` を返します。
    fn create_dir(path: &str) -> Result<(), FileSystemError>;

    /// 指定されたパスにあるディレクトリを削除します。
    ///
    /// # 引数
    ///
    /// * `path` - 削除するディレクトリのパスを表す文字列スライス。
    ///
    /// # 戻り値
    ///
    /// 成功時には `()` を返し、失敗時には `FileError` を返します。
    fn delete_dir(path: &str) -> Result<(), FileSystemError>;

    /// 指定されたディレクトリ内のファイルやサブディレクトリをリスト化します。
    ///
    /// # 引数
    ///
    /// * `path` - リスト化するディレクトリのパスを表す文字列スライス。
    ///
    /// # 戻り値
    ///
    /// 成功時には `FileMetadata` のベクターを返し、失敗時には `FileError` を返します。
    fn list_dir(path: &str) -> Result<Vec<FileMetadata>, FileSystemError>;

    /// 指定されたファイルまたはディレクトリのメタデータを取得します。
    ///
    /// # 引数
    ///
    /// * `path` - メタデータを取得する対象のパスを表す文字列スライス。
    ///
    /// # 戻り値
    ///
    /// 成功時には `FileMetadata` を返し、失敗時には `FileError` を返します。
    fn get_metadata(path: &str) -> Result<FileMetadata, FileSystemError>;

    /// 指定されたパスにあるファイルやディレクトリが存在するかどうかを確認します。
    ///
    /// # 引数
    ///
    /// * `path` - 存在確認する対象のパスを表す文字列スライス。
    ///
    /// # 戻り値
    ///
    /// 存在する場合は `true`、存在しない場合は `false` を返します。
    fn exists(path: &str) -> bool;

    /// ファイルやディレクトリの名前を変更します。
    ///
    /// # 引数
    ///
    /// * `old_path` - 現在のファイルやディレクトリのパス。
    /// * `new_path` - 新しい名前のパス。
    ///
    /// # 戻り値
    ///
    /// 成功時には `()` を返し、失敗時には `FileError` を返します。
    fn rename(old_path: &str, new_path: &str) -> Result<(), FileSystemError>;

    /// ファイルを別の場所にコピーします。
    ///
    /// # 引数
    ///
    /// * `src_path` - コピー元のファイルのパス。
    /// * `dest_path` - コピー先のファイルのパス。
    ///
    /// # 戻り値
    ///
    /// 成功時には `()` を返し、失敗時には `FileError` を返します。
    fn copy(src_path: &str, dest_path: &str) -> Result<(), FileSystemError>;


    /// 現在の作業ディレクトリを取得します。
    /// 
    /// # 戻り値
    /// 
    /// 現在の作業ディレクトリのパスを表す文字列を返します。
    fn get_working_directory() -> Result<String, FileSystemError>;

}
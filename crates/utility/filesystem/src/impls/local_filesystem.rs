use std::fs::{self, File as StdFile};
use std::io::{self, Read, Write};
use std::path::Path;
use crate::filesystem_handler::File;
use crate::filesystem_error::FileSystemError;
use crate::filesystem_hal::{FileMode, FileSystemHAL,FileMetadata};


/// ローカルファイルシステム用の `FileSystemHAL` の実装。
pub struct LocalFileSystem;

impl FileSystemHAL for LocalFileSystem {
    fn open_file(  path: &str, mode: FileMode) -> Result<File, FileSystemError> {
        let file: Result<StdFile, io::Error> = match mode {
            FileMode::Read => StdFile::open(path),
            FileMode::Write => StdFile::create(path),
            FileMode::Append => StdFile::options().append(true).open(path),
        };
        
        match file {
            Ok(handle) => Ok(File{ handle }),
            Err(e) => Err(FileSystemError::IoError(e)),
        }
    }

    fn read_file(file: &File) -> Result<Vec<u8>, FileSystemError> {
        let mut buffer = Vec::new();
        let mut handle = &file.handle;
        handle.read_to_end(&mut buffer).map_err(FileSystemError::IoError)?;
        Ok(buffer)
    }

    fn write_file(file: &File, data: &[u8]) -> Result<(), FileSystemError> {
        let mut handle = &file.handle;
        handle.write_all(data).map_err(FileSystemError::IoError)?;
        Ok(())
    }

    fn delete_file(path: &str) -> Result<(), FileSystemError> {
        fs::remove_file(path).map_err(FileSystemError::IoError)
    }

    fn create_dir(path: &str) -> Result<(), FileSystemError> {
        fs::create_dir(path).map_err(FileSystemError::IoError)
    }

    fn delete_dir(path: &str) -> Result<(), FileSystemError> {
        fs::remove_dir_all(path).map_err(FileSystemError::IoError)
    }

    fn list_dir(path: &str) -> Result<Vec<FileMetadata>, FileSystemError> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(path).map_err(FileSystemError::IoError)? {
            let entry = entry.map_err(FileSystemError::IoError)?;
            let metadata = entry.metadata().map_err(FileSystemError::IoError)?;
            entries.push(FileMetadata {
                size: metadata.len(),
                is_dir: metadata.is_dir(),
                created: metadata.created().ok(),
                modified: metadata.modified().ok(),
            });
        }
        Ok(entries)
    }

    fn get_metadata(path: &str) -> Result<FileMetadata, FileSystemError> {
        let metadata = fs::metadata(path).map_err(FileSystemError::IoError)?;
        Ok(FileMetadata {
            size: metadata.len(),
            is_dir: metadata.is_dir(),
            created: metadata.created().ok(),
            modified: metadata.modified().ok(),
        })
    }

    fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    fn rename(old_path: &str, new_path: &str) -> Result<(), FileSystemError> {
        fs::rename(old_path, new_path).map_err(FileSystemError::IoError)
    }

    fn copy(src_path: &str, dest_path: &str) -> Result<(), FileSystemError> {
        fs::copy(src_path, dest_path).map_err(FileSystemError::IoError)?;
        Ok(())
    }

    fn get_working_directory() -> Result<String, FileSystemError> {
        let path = std::env::current_dir().map_err(FileSystemError::IoError)?;
        Ok(path.to_str().unwrap().to_string())
    }
}
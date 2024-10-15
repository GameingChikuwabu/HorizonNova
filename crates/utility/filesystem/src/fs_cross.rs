use crate::impls::local_filesystem::LocalFileSystem;
use crate::filesystem_hal::{FileMode, FileSystemHAL,FileMetadata};
use crate::filesystem_handler::File;
use crate::filesystem_error::FileSystemError;

#[cfg(any(target_os = "linux", target_os = "macos",target_os = "windows"))]
type PlatformFileSystem = LocalFileSystem;

pub fn open_file(path: &str, mode: FileMode)->Result<File,FileSystemError>{
    PlatformFileSystem::open_file(path,mode)
}

pub fn read_file(file: &File)->Result<Vec<u8>,FileSystemError>{
    PlatformFileSystem::read_file(file)
}

pub fn write_file(file: &File, data: &[u8])->Result<(),FileSystemError>{
    PlatformFileSystem::write_file(file,data)
}

pub fn delete_file(path: &str)->Result<(),FileSystemError>{
    PlatformFileSystem::delete_file(path)
}

pub fn create_dir(path: &str)->Result<(),FileSystemError>{
    PlatformFileSystem::create_dir(path)
}

pub fn delete_dir(path: &str)->Result<(),FileSystemError>{
    PlatformFileSystem::delete_dir(path)
}

pub fn list_dir(path: &str)->Result<Vec<FileMetadata>,FileSystemError>{
    PlatformFileSystem::list_dir(path)
}

pub fn get_working_directory()->Result<String,FileSystemError>{
    PlatformFileSystem::get_working_directory()
}

pub fn get_metadata(path: &str)->Result<FileMetadata,FileSystemError>{
    PlatformFileSystem::get_metadata(path)
}
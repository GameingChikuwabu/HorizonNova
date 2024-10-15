#[cfg(test)]
mod tests{
    use filesystem::*;
    use filesystem_hal::FileMode;

    const TEST_FILE_PATH: &str = "tests/testfileassets/filesystemtest.txt";


    #[test]
    fn file_opene_test(){
        let file = fs_cross::open_file(TEST_FILE_PATH,FileMode::Read);
        assert!(file.is_ok());
    }

    #[test]
    fn file_read_test(){
        let file = fs_cross::open_file(TEST_FILE_PATH,FileMode::Read).unwrap();
        let data = fs_cross::read_file(&file);
        assert!(data.is_ok());
    }

    #[test]
    fn file_write_test(){
        let file = fs_cross::open_file(TEST_FILE_PATH,FileMode::Write).unwrap();
        let data = "test".as_bytes();
        let result = fs_cross::write_file(&file,data);
        assert!(result.is_ok());
    }

    #[test]
    fn file_delete_test(){
        let result = fs_cross::delete_file(TEST_FILE_PATH);
        assert!(result.is_ok());
    }

    #[test]
    fn dir_create_delete_test(){
        let result = fs_cross::create_dir("tests/testfileassets/testdir");
        assert!(result.is_ok());
        let result = fs_cross::delete_dir("tests/testfileassets/testdir");
        assert!(result.is_ok());
    }

    
    #[test]
    fn dir_list_test(){
        let result = fs_cross::list_dir("tests/testfileassets");
        assert!(result.is_ok());
    }

    #[test]
    fn working_directory_test(){
        let result = fs_cross::get_working_directory();
        assert!(result.is_ok());
    }

    #[test]
    fn metadata_test(){
        let result = fs_cross::get_metadata(TEST_FILE_PATH);
        assert!(result.is_ok());
    }
}
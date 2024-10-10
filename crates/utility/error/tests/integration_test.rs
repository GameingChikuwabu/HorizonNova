#[cfg(test)]
mod tests {
    use error::*;

    #[derive(Error,Debug)]
    enum TestError{
        #[error("Error1")]
        Error1,
        #[error("Error2: {0}")]
        Error2(String),
    }

    fn test_function() -> Result<(),TestError>{
        Err(TestError::Error1)
    }

    fn test_error_function() -> Result<(),TestError>{
        Err(TestError::Error2("Error2".to_string()))
    }

    #[test]
    fn error_test(){
        let result = test_function();
        assert_eq!(result.is_err(),true);
        assert_eq!(result.unwrap_err().to_string(),"Error1");

        let result = test_error_function();
        assert_eq!(result.is_err(),true);
        assert_eq!(result.unwrap_err().to_string(),"Error2: Error2");
    }
}
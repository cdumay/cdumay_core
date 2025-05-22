#[cfg(test)]
mod test {
    use cdumay_core::Result;
    #[test]
    fn test_simple_result() {
        fn double(x: i32) -> Result<i32> {
            Ok(x * 2).into()
        }
        let result = double(2);
        assert_eq!(result, Result::Ok(4));
    }
    #[test]
    fn test_map() {
        let result: Result<i32> = Result::Ok(2);
        let mapped = result.map(|x| x * 10);
        assert!(matches!(mapped, Result::Ok(20)));
    }
    
}
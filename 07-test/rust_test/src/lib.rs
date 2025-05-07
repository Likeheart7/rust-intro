pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // 成功的测试
    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // 产生panic的测试会失败
    #[test]
    fn panic() {
        panic!("this is a panic");
    }

    #[test]
    #[should_panic(expected = "will contains panic message")] // 标注#[should_panic]的测试在没有panic的时候提示异常
    fn test_failed() {
        let r = add(10, 19);
        assert_eq!(r, 29, "result is not equal to r");
    }

    #[test]
    #[ignore = "this is a ignored tests"]
    fn test_ignore() {
        assert_eq!(1, 2);
    }
}

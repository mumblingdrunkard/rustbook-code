pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_test() {
        assert_eq!(add_two(5), 7);
        assert_eq!(add_two(3), 5);
        assert_eq!(add_two(-4), -2);
    }
}

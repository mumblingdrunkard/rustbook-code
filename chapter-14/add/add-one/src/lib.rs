pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_test() {
        assert_eq!(add_one(5), 6);
        assert_eq!(add_one(3), 4);
        assert_eq!(add_one(-4), -3);
    }
}

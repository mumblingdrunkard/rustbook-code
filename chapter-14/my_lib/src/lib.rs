//! # My Lib
//! `my_lib` is a collection of utilities to make performing certain
//! calculations more convenient

/// Adds one to the given number
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_lib::add_one(arg);
///
/// assert_eq!(arg + 1, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

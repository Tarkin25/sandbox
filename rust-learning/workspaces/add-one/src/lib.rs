
/// Adds one to a given number.
///
/// # Examples
/// ```
/// let a = 1;
/// let result = add_one::add_one(1);
///
/// assert_eq!(2, result);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_one_to_one_returns_2() {
        assert_eq!(2, add_one(1));
    }

}

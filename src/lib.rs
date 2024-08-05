pub fn gray_code(n: i32) -> Vec<i32> {
        let size = 1 << n;
        (0..size).map(|num| num ^ (num >> 1)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(gray_code(2), [0, 1, 3, 2]);
        debug_assert_eq!(gray_code(1), [0, 1]);
    }

    #[test]
    fn test() {}
}

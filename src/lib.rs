mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn title_to_number(column_title: &str) -> i32 {
    column_title
        .bytes()
        .fold(0, |acc, byte| acc * 26 + (byte - b'A' + 1) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(title_to_number("A"), 1);
        debug_assert_eq!(title_to_number("AB"), 28);
        debug_assert_eq!(title_to_number("ZY"), 701);
    }

    #[test]
    fn test() {}
}

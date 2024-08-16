mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn convert_to_title(column_number: i32) -> String {
    let mut chs = vec![];
    let mut num = column_number;
    while num > 0 {
        chs.push((num - 1) % 26);
        num = (num - 1) / 26;
    }
    chs.reverse();
    chs.into_iter()
        .map(|n| char::from(n as u8 + b'A'))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(convert_to_title(1), "A");
        // debug_assert_eq!(convert_to_title(28), "AB");
        debug_assert_eq!(convert_to_title(701), "ZY");
    }

    #[test]
    fn test() {
        debug_assert_eq!(convert_to_title(52), "AZ");
    }
}

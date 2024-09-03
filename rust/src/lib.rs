mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn get_lucky(s: &str, k: i32) -> i32 {
    let mut num: i32 = s
        .bytes()
        .map(|b| {
            let n = i32::from(b - b'a' + 1);
            if n >= 10 {
                n % 10 + n / 10
            } else {
                n
            }
        })
        .sum();
    for _ in 0..k - 1 {
        if num < 10 {
            return num;
        }
        let mut temp = num;
        num = 0;
        while temp > 0 {
            num += temp % 10;
            temp /= 10;
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(get_lucky("iiii", 1), 36);
        debug_assert_eq!(get_lucky("leetcode", 2), 6);
        debug_assert_eq!(get_lucky("zbax", 2), 8);
    }

    #[test]
    fn test() {}
}

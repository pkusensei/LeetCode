mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn count_digit_one(n: i32) -> i32 {
    let n = i64::from(n);
    let mut count = 0;
    let mut factor = 1;

    while factor <= n {
        let lower = n - n / factor * factor;
        let curr = n / factor % 10;
        let higher = n / (factor * 10);
        match curr {
            0 => count += higher * factor,
            1 => count += higher * factor + lower + 1,
            _ => count += (higher + 1) * factor,
        }
        factor *= 10;
    }
    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(count_digit_one(13), 6);
        debug_assert_eq!(count_digit_one(0), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(count_digit_one(824883294), 767944060);
    }
}

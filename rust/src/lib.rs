mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn chalk_replacer(chalk: &[i32], k: i32) -> i32 {
    let sum: i64 = chalk.iter().map(|&n| i64::from(n)).sum();
    let mut k = (k as i64) % sum;
    for (i, &num) in chalk.iter().enumerate() {
        if num as i64 > k {
            return i as i32;
        } else {
            k -= num as i64
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(chalk_replacer(&[5, 1, 5], 22), 0);
        debug_assert_eq!(chalk_replacer(&[3, 4, 1, 2], 25), 1);
    }

    #[test]
    fn test() {}
}

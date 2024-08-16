mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn largest_number(mut nums: Vec<i32>) -> String {
    nums.sort_unstable_by(|&a, &b| {
        use std::cmp::Ordering::*;
        if a == b {
            return Equal;
        }
        if a == 0 {
            return Less;
        }
        if b == 0 {
            return Greater;
        }

        let (a_len, b_len) = (a.ilog10() + 1, b.ilog10() + 1);
        let (a, b) = (a as i64, b as i64);
        (a * 10i64.pow(b_len) + b).cmp(&(b * 10i64.pow(a_len) + a))
    });
    while nums.last().is_some_and(|&n| n == 0) {
        nums.pop(); // Pop zeros at tail, i.e leading zeros
    }
    if nums.is_empty() {
        "0".to_string()
    } else {
        nums.reverse();
        nums.into_iter().map(|n| n.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(largest_number(vec![10, 2]), "210");
        debug_assert_eq!(largest_number(vec![3, 30, 34, 5, 9]), "9534330");
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            largest_number(vec![999999998, 999999997, 999999999]),
            "999999999999999998999999997"
        );
        debug_assert_eq!(
            largest_number(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            "9876543210"
        );
        debug_assert_eq!(largest_number(vec![0, 0]), "0");
    }
}

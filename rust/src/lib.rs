mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_ratings(units: Vec<Vec<i32>>) -> i64 {
    let cols = units[0].len();
    if cols == 1 {
        return units.iter().map(|v| i64::from(v[0])).sum();
    }
    let mut res = i32::MAX >> 1;
    let mut arr = vec![];
    for [min1, min2] in units.iter().map(|v| find(v)) {
        res = res.min(min1);
        arr.push(min2);
    }
    arr.sort_unstable();
    i64::from(res) + arr[1..].iter().map(|&v| i64::from(v)).sum::<i64>()
}

// [min, second_min]
fn find(nums: &[i32]) -> [i32; 2] {
    let [mut min1, mut min2] = [i32::MAX >> 1; 2];
    for &num in nums {
        if num <= min1 {
            min2 = min1;
            min1 = num
        } else if num < min2 {
            min2 = num
        }
    }
    [min1, min2]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}

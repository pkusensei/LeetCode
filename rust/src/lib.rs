mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum(nums: Vec<i32>) -> i32 {
    fn find(mut num: i32) -> i32 {
        let mut d = 0;
        while num > 0 {
            d = d.max(num % 10);
            num /= 10
        }
        d
    }
    let mut ds = [const { vec![] }; 10];
    for &num in nums.iter() {
        let d = find(num);
        ds[d as usize].push(num);
    }
    let mut res = -1;
    for v in ds.iter_mut().rev() {
        if v.len() > 1 {
            v.sort_unstable_by(|a, b| b.cmp(a));
            res = res.max(v[0] + v[1]);
        }
    }
    res
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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

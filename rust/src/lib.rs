mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let arr: Vec<_> = s
        .bytes()
        .zip(t.bytes())
        .map(|(a, b)| i32::from(a.abs_diff(b)))
        .collect();
    let mut res = None;
    let mut sum = 0;
    let mut left = 0;
    for (right, &num) in arr.iter().enumerate() {
        sum += num;
        while sum > max_cost {
            sum -= arr[left];
            left += 1;
        }
        res = res.max(Some(1 + right - left))
    }
    res.map(|v| v as i32).unwrap_or(0)
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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

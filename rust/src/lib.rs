mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_time(s: String, order: Vec<i32>, k: i32) -> i32 {
    let n = s.len();
    let mut s = s.into_bytes();
    let mut left = 0;
    let mut right = n;
    while left < right {
        let mid = left + (right - left) / 2;
        if count(&mut s, &order, mid) >= k as usize {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    if left < n { left as i32 } else { -1 }
}

fn count(s: &mut [u8], order: &[i32], mid: usize) -> usize {
    let n = s.len();
    for &i in &order[..=mid] {
        s[i as usize] = b'*';
    }
    let v = s
        .split(|&b| b == b'*')
        .map(|w| {
            let v = w.len();
            v * (1 + v) / 2
        })
        .sum::<usize>();
    let res = n * (1 + n) / 2 - v;
    for &i in &order[..=mid] {
        s[i as usize] = b'#';
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

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_min_swaps(num: String, k: i32) -> i32 {
    let mut perm = num.as_bytes().to_vec();
    for _ in 0..k {
        next_perm(&mut perm);
    }
    let mut num = num.into_bytes();
    let mut res = 0;
    for (i1, &target) in perm.iter().enumerate() {
        if num[i1] != target {
            let mut i2 = 1 + i1;
            while num.get(i2).is_some_and(|&v| v != target) {
                i2 += 1;
            }
            num[i1..=i2].rotate_right(1);
            res += i2 - i1
        }
    }
    res as _
}

fn next_perm(s: &mut [u8]) {
    let n = s.len();
    let mut left = None;
    // Find rightmost i where [i]<[i+1]
    for idx in (0..n - 1).rev() {
        if s[idx] < s[idx + 1] {
            left = Some(idx);
            break;
        }
    }
    let Some(left) = left else {
        s.sort_unstable(); // already last perm
        return;
    };
    // Find and swap rightmost [i] where [left]<[i]
    for right in (1 + left..n).rev() {
        if s[left] < s[right] {
            s.swap(left, right);
            break;
        }
    }
    s[1 + left..].reverse();
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
    fn basics() {
        assert_eq!(get_min_swaps("5489355142".into(), 4), 2);
        assert_eq!(get_min_swaps("11112".into(), 4), 4);
        assert_eq!(get_min_swaps("00123".into(), 1), 1);
    }

    #[test]
    fn test() {}
}

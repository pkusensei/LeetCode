mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn closest_target(words: &[&str], target: &str, start_index: i32) -> i32 {
    let n = words.len();
    let start = start_index as usize;
    let mut idx = start;
    let mut a = 0;
    while words[idx] != target {
        idx = (1 + idx) % n;
        a += 1;
        if idx == start {
            return -1;
        }
    }
    let mut b = 0;
    idx = start;
    while words[idx] != target {
        idx = (idx + n - 1) % n;
        b += 1;
        if idx == start {
            break;
        }
    }
    a.min(b)
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
        assert_eq!(closest_target(&["a", "b", "leetcode"], "leetcode", 0), 1);
    }

    #[test]
    fn test() {}
}

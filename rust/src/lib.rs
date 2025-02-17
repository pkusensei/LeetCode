mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_product(s: String) -> i64 {
    let mut s = s.into_bytes();
    let prefix = manachers(&s);
    s.reverse();
    let mut suffix = manachers(&s);
    suffix.reverse();
    prefix
        .into_iter()
        .zip(suffix[1..].iter())
        .map(|(a, b)| (a * b) as i64)
        .max()
        .unwrap_or(1)
}

fn manachers(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut radius = vec![0; n];
    let mut res = vec![1; n];
    let [mut center, mut right_bound] = [0, 0];
    for idx in 0..n {
        if idx < right_bound {
            let mirror = 2 * center - idx;
            radius[idx] = radius[mirror].min(right_bound - idx);
        }
        let mut left = idx as i64 - (1 + radius[idx]) as i64;
        let mut right = idx + (1 + radius[idx]);
        while left >= 0 && right < n && s[left as usize] == s[right] {
            radius[idx] += 1;
            res[idx + radius[idx]] = res[idx + radius[idx]].max(1 + 2 * radius[idx]);
            left -= 1;
            right += 1;
        }
        if idx + radius[idx] > right_bound {
            center = idx;
            right_bound = idx + radius[idx];
        }
    }
    // Find prefix max length
    for idx in 1..n {
        res[idx] = res[idx].max(res[idx - 1]);
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
    fn basics() {
        assert_eq!(max_product("ababbb".into()), 9);
        assert_eq!(max_product("zaaaxbbby".into()), 9);
    }

    #[test]
    fn test() {
        assert_eq!(
            max_product("ggbswiymmlevedhkbdhntnhdbkhdevelmmyiwsbgg".into()),
            45
        );
    }
}

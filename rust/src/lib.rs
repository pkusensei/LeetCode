mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn beautiful_indices(s: &str, a: &str, b: &str, k: i32) -> Vec<i32> {
    let ns = s.len();
    let k = k as usize;
    let mut ia = s.find(a).unwrap_or(ns);
    let mut ib = s.find(b).unwrap_or(ns);
    let mut res = vec![];
    while ia < ns && ib < ns {
        if ia < ib.saturating_sub(k) {
            ia += ib - ia - k;
            let Some(v) = s[ia..].find(a) else {
                break;
            };
            ia += v;
        } else if ib + k < ia {
            ib += ia - ib - k;
            let Some(v) = s[ib..].find(b) else {
                break;
            };
            ib += v;
        } else {
            res.push(ia as i32);
            ia += 1;
            let Some(v) = s[ia..].find(a) else {
                break;
            };
            ia += v;
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
    fn basics() {
        assert_eq!(
            beautiful_indices("isawsquirrelnearmysquirrelhouseohmy", "my", "squirrel", 15),
            [16, 33]
        );
        assert_eq!(beautiful_indices("abcd", "a", "a", 4), [0]);
    }

    #[test]
    fn test() {}
}

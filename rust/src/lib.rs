mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::izip;

pub fn transform_str(s: &str, strs: &[&str]) -> Vec<bool> {
    let pref_ones = s.bytes().fold(vec![], |mut acc, b| {
        let curr = i32::from(b - b'0');
        acc.push(curr + acc.last().unwrap_or(&0));
        acc
    });
    let n = s.len();
    let mut res = vec![];
    'out: for target in strs.iter() {
        let mut prefix = 0;
        let mut wild = 0;
        for (i, b) in target.bytes().enumerate() {
            prefix += i32::from(b == b'1');
            wild += i32::from(b == b'?');
            if prefix > pref_ones[i] {
                res.push(false);
                continue 'out;
            }
        }
        if prefix + wild < pref_ones[n - 1] {
            res.push(false);
            continue;
        }
        let need = pref_ones[n - 1] - prefix;
        let mut t = target.as_bytes().to_vec();
        let mut used_wild = 0;
        for b in t.iter_mut().rev() {
            if *b == b'?' {
                if used_wild < need {
                    *b = b'1';
                    used_wild += 1;
                } else {
                    *b = b'0'
                }
            }
        }
        let [mut pref_s, mut pref_t] = [0, 0];
        for (sb, &tb) in izip!(s.bytes(), t.iter()) {
            pref_s += i32::from(sb == b'1');
            pref_t += i32::from(tb == b'1');
            if pref_s < pref_t {
                res.push(false);
                continue 'out;
            }
        }
        res.push(true);
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
    fn basics() {
        assert_eq!(
            transform_str("101", &["1?1", "0?1", "0?0"]),
            [true, true, false]
        );
    }

    #[test]
    fn test() {
        assert_eq!(transform_str("01", &["?0"]), [false]);
    }
}

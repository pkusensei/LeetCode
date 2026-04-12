mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_balanced(s: String) -> i32 {
    use std::collections::HashMap;
    let n = s.len();
    let [mut pref_ones, mut pref_zeros] = [const { vec![] }; 2];
    for b in s.bytes() {
        pref_ones.push(i32::from(b == b'1') + pref_ones.last().unwrap_or(&0));
        pref_zeros.push(i32::from(b == b'0') + pref_zeros.last().unwrap_or(&0));
    }
    let mut sum = 0;
    // sum=0
    let mut prefix = HashMap::from([(0, -1)]);
    let mut pref_has0 = HashMap::new();
    let mut pref_has1 = HashMap::new();
    let mut res = 0;
    // 1 1 1 0 1 0
    // 1 2 3 2 3 2
    for (idx, b) in s.bytes().enumerate() {
        sum += if b == b'1' { 1 } else { -1 };
        if let Some(prev) = prefix.get(&sum) {
            res = res.max(idx as i32 - prev);
        }
        // need 0's
        if let Some(&prev) = prefix.get(&(sum - 2)) {
            let window0 = pref_zeros[idx]
                - if prev >= 0 {
                    pref_zeros[prev as usize]
                } else {
                    0
                };
            if pref_zeros[n - 1] > window0 {
                res = res.max(idx as i32 - prev);
            } else if let Some(v) = pref_has0.get(&(sum - 2)) {
                res = res.max(idx as i32 - v);
            }
        }
        // need 1's
        if let Some(&prev) = prefix.get(&(sum + 2)) {
            let window1 = pref_ones[idx]
                - if prev >= 0 {
                    pref_ones[prev as usize]
                } else {
                    0
                };
            if pref_ones[n - 1] > window1 {
                res = res.max(idx as i32 - prev);
            } else if let Some(v) = pref_has1.get(&(sum + 2)) {
                res = res.max(idx as i32 - v);
            }
        }
        prefix.entry(sum).or_insert(idx as i32);
        if pref_zeros[idx] > 0 {
            pref_has0.entry(sum).or_insert(idx as i32);
        }
        if pref_ones[idx] > 0 {
            pref_has1.entry(sum).or_insert(idx as i32);
        }
    }
    res as i32
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

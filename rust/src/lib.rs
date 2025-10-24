mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn next_beautiful_number(n: i32) -> i32 {
    let s = {
        let mut s = vec![];
        let mut n = n;
        while n > 0 {
            s.push(n % 10);
            n /= 10;
        }
        s.push(0); // leading zero
        s.reverse();
        s
    };
    let mut freq = [0; 8];
    dfs(&s, 0, true, 0, true, &mut freq)
}

fn dfs(s: &[i32], idx: usize, tight: bool, num: i32, leading_zero: bool, freq: &mut [i32]) -> i32 {
    const M: i32 = 10_000_000;
    if idx >= s.len() {
        if !tight
            && !leading_zero
            && freq
                .iter()
                .enumerate()
                .skip(1)
                .all(|(i, &v)| v == 0 || v == i as i32)
        {
            return num;
        }
        return M;
    }
    let lower = if tight { s[idx] } else { 1 };
    let mut res = M;
    for d in lower..=7 {
        if !leading_zero && d == 0 {
            continue;
        }
        let ntight = tight && d == lower;
        let nleading = leading_zero && d == 0;
        freq[d as usize] += 1;
        res = res.min(dfs(s, 1 + idx, ntight, num * 10 + d, nleading, freq));
        freq[d as usize] -= 1;
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
        assert_eq!(next_beautiful_number(1000), 1333);
        assert_eq!(next_beautiful_number(3000), 3133)
    }

    #[test]
    fn test() {}
}

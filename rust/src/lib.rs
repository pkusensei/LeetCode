mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn total_waviness(num1: i64, num2: i64) -> i64 {
    f(num2) - f(num1 - 1)
}

fn f(num: i64) -> i64 {
    if num < 101 {
        return 0;
    }
    let s = num.to_string().into_bytes();
    let n = s.len();
    let mut memo = vec![[[[[None; 10]; 10]; 2]; 2]; n];
    dfs(&s, 0, true, true, b'0', b'0', &mut memo)[0]
}

fn dfs(
    s: &[u8],
    idx: usize,
    tight: bool,
    leading: bool,
    left: u8,
    mid: u8,
    memo: &mut [[[[[Option<[i64; 2]>; 10]; 10]; 2]; 2]],
) -> [i64; 2] {
    if idx >= s.len() {
        return [0, 1];
    }
    if let Some(v) = memo[idx][usize::from(tight)][usize::from(leading)][usize::from(left - b'0')]
        [usize::from(mid - b'0')]
    {
        return v;
    }
    let upper = if tight { s[idx] } else { b'9' };
    let mut wav = 0;
    let mut count = 0;
    for d in b'0'..=upper {
        let ntight = tight && d == upper;
        let nleading = leading && mid == b'0';
        let [nwav, ncount] = dfs(s, 1 + idx, ntight, nleading, mid, d, memo);
        wav += nwav;
        count += ncount;
        if !leading {
            if left < mid && mid > d {
                wav += ncount
            }
            if left > mid && mid < d {
                wav += ncount
            }
        }
    }
    memo[idx][usize::from(tight)][usize::from(leading)][usize::from(left - b'0')]
        [usize::from(mid - b'0')] = Some([wav, count]);
    [wav, count]
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

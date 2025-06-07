mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost_good_caption(caption: &str) -> String {
    let (s, n) = (caption.as_bytes(), caption.len());
    if n < 3 {
        return String::new();
    }
    let mut memo = vec![[[-1; 3]; 27]; n];
    let cost = dfs(s, 0, 26, 2, &mut memo);
    if cost >= INF {
        return String::new();
    }
    let mut res = build(s, &memo, 0, 26, 2, cost);
    res.reverse();
    String::from_utf8(res).unwrap()
}

const INF: i32 = i32::MAX / 2;
fn dfs(s: &[u8], idx: usize, prev_b: usize, streak: usize, memo: &mut [[[i32; 3]; 27]]) -> i32 {
    if idx >= s.len() {
        return if streak == 2 { 0 } else { INF };
    }
    if memo[idx][prev_b][streak] > -1 {
        return memo[idx][prev_b][streak];
    }
    let mut res = INF;
    if streak == 2 {
        // starts new chunk or stays in
        for (bi, b) in (b'a'..=b'z').enumerate() {
            let cost = i32::from(b.abs_diff(s[idx]));
            res = res.min(cost + dfs(s, 1 + idx, bi, if bi == prev_b { 2 } else { 0 }, memo));
        }
    } else {
        // expand previous chunk
        let cost = i32::from((prev_b as u8 + b'a').abs_diff(s[idx]));
        res = res.min(cost + dfs(s, 1 + idx, prev_b, 1 + streak, memo));
    }
    memo[idx][prev_b][streak] = res;
    res
}

fn build(
    s: &[u8],
    memo: &[[[i32; 3]; 27]],
    idx: usize,
    prev_b: usize,
    streak: usize,
    target: i32,
) -> Vec<u8> {
    let n = s.len();
    if idx >= n {
        return Vec::with_capacity(n);
    }
    if streak == 2 {
        for (bi, b) in (b'a'..=b'z').enumerate() {
            let cost = i32::from(b.abs_diff(s[idx]));
            let next_streak = if bi == prev_b { 2 } else { 0 };
            let next_cost = if 1 + idx >= n {
                if next_streak == 2 { 0 } else { INF }
            } else {
                memo[1 + idx][bi][next_streak]
            };
            if cost + next_cost == target {
                let mut res = build(s, memo, 1 + idx, bi, next_streak, next_cost);
                res.push(b);
                return res;
            }
        }
    } else {
        let b = prev_b as u8 + b'a';
        let cost = i32::from(b.abs_diff(s[idx]));
        let next_streak = 1 + streak;
        let next_cost = if 1 + idx >= n {
            if next_streak == 2 { 0 } else { INF }
        } else {
            memo[1 + idx][prev_b][next_streak]
        };
        if cost + next_cost == target {
            let mut res = build(s, memo, 1 + idx, prev_b, next_streak, next_cost);
            res.push(b);
            return res;
        }
    }
    vec![] // unreachable
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
        assert_eq!(min_cost_good_caption("cdcd"), "cccc");
        assert_eq!(min_cost_good_caption("aca"), "aaa");
        assert_eq!(min_cost_good_caption("bc"), "");
    }

    #[test]
    fn test() {}
}

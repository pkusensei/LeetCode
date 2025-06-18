mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_numbers(l: &str, r: &str, b: i32) -> i32 {
    let mut s1 = parse(&l, b.into());
    for v in s1.iter_mut().rev() {
        if *v == 0 {
            *v = b as u8 - 1;
        } else {
            *v -= 1;
            break;
        }
    }
    let s2 = parse(&r, b.into());
    let a = solve(&s1, b as u8);
    let b = solve(&s2, b as u8);
    (b - a).rem_euclid(M)
}

const M: i32 = 1_000_000_007;

fn parse(s: &str, b: i64) -> Vec<u8> {
    let mut curr = vec![0];
    for d in s.bytes().map(|ch| ch - b'0') {
        let mut carry = 0;
        for i in curr.iter_mut() {
            let prod = (*i) * 10 + carry;
            *i = prod % b;
            carry = prod / b;
        }
        while carry > 0 {
            curr.push(carry % b);
            carry /= b;
        }
        carry = i64::from(d);
        for i in curr.iter_mut() {
            let sum = *i + carry;
            *i = sum % b;
            carry = sum / b;
            if carry < 1 {
                break;
            }
        }
        while carry > 0 {
            curr.push(carry % b);
            carry /= b;
        }
    }
    let mut res: Vec<u8> = curr.iter().rev().map(|&v| v as u8).collect();
    if res.is_empty() {
        res = vec![0]
    }
    res
}

fn solve(ds: &[u8], b: u8) -> i32 {
    let n = ds.len();
    dfs(ds, b, 0, true, 0, &mut vec![[[-1; 10]; 2]; n])
}

fn dfs(nums: &[u8], b: u8, idx: usize, tight: bool, prev: u8, memo: &mut [[[i32; 10]; 2]]) -> i32 {
    if idx >= nums.len() {
        return 1;
    }
    if memo[idx][usize::from(tight)][usize::from(prev)] > -1 {
        return memo[idx][usize::from(tight)][usize::from(prev)];
    }
    let upper = if tight { nums[idx] } else { b - 1 };
    let mut res = 0;
    for d in prev..=upper {
        let next_tight = tight && d == upper;
        res += dfs(nums, b, 1 + idx, next_tight, d, memo);
        res %= M;
    }
    memo[idx][usize::from(tight)][usize::from(prev)] = res;
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
        assert_eq!(count_numbers("23", "28", 8), 3);
        assert_eq!(count_numbers("2", "7", 2), 2);
    }

    #[test]
    fn test() {
        assert_eq!(
            count_numbers("3017011", "6014684453239676968884889861183768278549", 4),
            52032
        );
    }
}

mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::LazyLock,
};

pub fn with_precompute(l: i64, r: i64) -> i64 {
    let mut res = cnt_driver(r) - cnt_driver(l - 1);
    let a = GOODS.partition_point(|&v| v < l);
    let b = GOODS.partition_point(|&v| v <= r);
    res += (b - a) as i64;
    for &num in &GOODS[a..b] {
        let sum = num
            .to_string()
            .bytes()
            .map(|b| i32::from(b - b'0'))
            .sum::<i32>();
        if GOOD_SUMS.contains(&sum) {
            res -= 1;
        }
    }
    res
}

fn cnt_driver(num: i64) -> i64 {
    if num == 0 {
        return 0;
    }
    let s = num
        .to_string()
        .bytes()
        .map(|b| i32::from(b - b'0'))
        .collect_vec();
    let n = s.len();
    cnt(&s, 0, true, 0, &mut vec![[[-1; 145]; 2]; n])
}

fn cnt(s: &[i32], idx: usize, tight: bool, sum: i32, memo: &mut [[[i64; 145]; 2]]) -> i64 {
    if idx >= s.len() {
        return i64::from(GOOD_SUMS.contains(&sum));
    }
    if memo[idx][usize::from(tight)][sum as usize] > -1 {
        return memo[idx][usize::from(tight)][sum as usize];
    }
    let upper = if tight { s[idx] } else { 9 };
    let mut res = 0;
    for d in 0..=upper {
        let ntight = tight && d == upper;
        res += cnt(s, 1 + idx, ntight, d + sum, memo);
    }
    memo[idx][usize::from(tight)][sum as usize] = res;
    res
}

static GOODS: LazyLock<Vec<i64>> = LazyLock::new(precompute_good);
static GOOD_SUMS: LazyLock<Vec<i32>> = LazyLock::new(precompute_sums);

fn precompute_good() -> Vec<i64> {
    // (num, last_digit, dir)
    let mut queue = VecDeque::new();
    for d in 1..=9 {
        queue.push_back((d, d, 0));
        queue.push_back((d, d, 1));
    }
    let mut res = HashSet::new();
    while let Some((num, d, dir)) = queue.pop_front() {
        res.insert(num);
        for nd in 0..=9 {
            let nnum = 10 * num + nd;
            if dir == 0 && d < nd {
                queue.push_back((nnum, nd, dir));
            }
            if dir == 1 && d > nd {
                queue.push_back((nnum, nd, dir));
            }
        }
    }
    res.into_iter().sorted_unstable().collect()
}

fn precompute_sums() -> Vec<i32> {
    (1..=144)
        .filter(|&v| is_monotone(v.to_string().as_bytes()))
        .collect()
}

pub fn count_fancy(l: i64, r: i64) -> i64 {
    solve(r) - solve(l - 1)
}

fn solve(num: i64) -> i64 {
    if num == 0 {
        return 0;
    }
    let s = num
        .to_string()
        .bytes()
        .map(|b| i32::from(b - b'0'))
        .collect_vec();
    dfs(&s, (0, Dir::None, true, true, -1, 0), &mut HashMap::new())
}

fn dfs(s: &[i32], key: Key, memo: &mut HashMap<Key, i64>) -> i64 {
    let (idx, dir, tight, leading, prev, sum) = key;
    if idx >= s.len() {
        if leading {
            return 0;
        }
        if matches!(dir, Dir::Inc | Dir::Dec) {
            return 1;
        }
        return i64::from(is_monotone(sum.to_string().as_bytes()));
    }
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let upper = if tight { s[idx] } else { 9 };
    let mut res = 0;
    for d in 0..=upper {
        let ntight = tight && d == upper;
        let nleading = leading && d == 0;
        if nleading {
            res += dfs(s, (1 + idx, dir, ntight, nleading, prev, sum), memo);
        } else if leading {
            res += dfs(s, (1 + idx, dir, ntight, nleading, d, d + sum), memo);
        } else {
            let ndir = match dir {
                Dir::None => match prev.cmp(&d) {
                    std::cmp::Ordering::Less => Dir::Inc,
                    std::cmp::Ordering::Equal => Dir::Broken,
                    std::cmp::Ordering::Greater => Dir::Dec,
                },
                Dir::Inc => {
                    if prev < d {
                        Dir::Inc
                    } else {
                        Dir::Broken
                    }
                }
                Dir::Dec => {
                    if prev > d {
                        Dir::Dec
                    } else {
                        Dir::Broken
                    }
                }
                Dir::Broken => dir,
            };
            res += dfs(s, (1 + idx, ndir, ntight, nleading, d, d + sum), memo);
        }
    }
    memo.insert(key, res);
    res
}

type Key = (usize, Dir, bool, bool, i32, i32);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    None,
    Inc,
    Dec,
    Broken,
}

fn is_monotone(s: &[u8]) -> bool {
    s.is_sorted_by(|a, b| a < b) || s.is_sorted_by(|a, b| a > b)
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
        assert_eq!(count_fancy(12340, 12341), 1);
        assert_eq!(count_fancy(8, 10), 3);
        assert_eq!(count_fancy(123456788, 123456788), 0);

        assert_eq!(with_precompute(8, 10), 3);
        assert_eq!(with_precompute(12340, 12341), 1);
        assert_eq!(with_precompute(123456788, 123456788), 0);
    }

    #[test]
    fn test() {
        assert_eq!(count_fancy(14, 22), 9);

        assert_eq!(with_precompute(14, 22), 9);
    }
}

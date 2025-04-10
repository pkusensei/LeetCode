mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: &str) -> i64 {
    less_than(finish, limit, &s) - less_than(start - 1, limit, &s)
}

fn less_than(num: i64, limit: i32, s: &str) -> i64 {
    let num = num.to_string();
    let mut memo = vec![[-1; 2]; num.len()];
    dfs(&num, limit, s, 0, 1, &mut memo)
}

fn dfs(num: &str, limit: i32, s: &str, idx: usize, tight: usize, memo: &mut [[i64; 2]]) -> i64 {
    let n = num.len();
    if idx >= n {
        return 1;
    }
    if n < s.len() {
        return 0;
    }
    if memo[idx][tight] > -1 {
        return memo[idx][tight];
    }
    let len = n - s.len();
    let num_d = i32::from(num.as_bytes()[idx] - b'0');
    let upper = if tight > 0 { num_d.min(limit) } else { limit };
    let mut res = 0;
    if idx >= len {
        match upper.cmp(&i32::from(s.as_bytes()[idx - len] - b'0')) {
            std::cmp::Ordering::Less => return 0,
            std::cmp::Ordering::Greater => return 1,
            std::cmp::Ordering::Equal => res += dfs(num, limit, s, 1 + idx, tight & 1, memo),
        }
    } else {
        for d in 0..=upper {
            let next_tight = tight & usize::from(d == num_d);
            res += dfs(num, limit, s, 1 + idx, next_tight, memo);
        }
    }
    memo[idx][tight] = res;
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
        assert_eq!(number_of_powerful_int(1, 6000, 4, "124"), 5);
        assert_eq!(number_of_powerful_int(15, 215, 6, "10"), 2);
        assert_eq!(number_of_powerful_int(1000, 2000, 4, "3000"), 0);
    }

    #[test]
    fn test() {}
}

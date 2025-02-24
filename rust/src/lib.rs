mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn next_beautiful_number(n: i32) -> i32 {
    let mut n = 1 + n;
    while !check(n) {
        n += 1;
    }
    n
}

fn check(mut n: i32) -> bool {
    let mut count = [0; 10];
    while n > 0 {
        let d = (n % 10) as usize;
        if d == 0 {
            return false;
        }
        count[d] += 1;
        if count[d] > d {
            return false;
        }
        n /= 10;
    }
    count.iter().enumerate().all(|(i, &c)| c == 0 || i == c)
}

pub fn with_backtrack(n: i32) -> i32 {
    fn dfs(curr: i32, min: i32, len: i32, count: &mut [i32; 10]) -> Option<i32> {
        if len == 0 {
            if count
                .iter()
                .enumerate()
                .any(|(i, &c)| c > 0 && c != i as i32)
            {
                return None;
            }
            return if curr > min { Some(curr) } else { None };
        }
        let mut res = i32::MAX;
        for i in 1..=9 {
            if count[i] < i as i32 {
                count[i] += 1;
                if let Some(v) = dfs(curr * 10 + i as i32, min, len - 1, count) {
                    res = res.min(v);
                }
                count[i] -= 1;
            }
        }
        if res == i32::MAX {
            None
        } else {
            Some(res)
        }
    }

    let len = 1 + n.ilog10() as i32;
    let mut count = [0; 10];
    dfs(0, n, len, &mut count)
        .or_else(|| dfs(0, n, 1 + len, &mut count))
        .unwrap()
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
        assert_eq!(next_beautiful_number(1), 22);
        assert_eq!(next_beautiful_number(1000), 1333);
        assert_eq!(next_beautiful_number(3000), 3133);

        assert_eq!(with_backtrack(1), 22);
        assert_eq!(with_backtrack(1000), 1333);
        assert_eq!(with_backtrack(3000), 3133);
    }

    #[test]
    fn test() {}
}

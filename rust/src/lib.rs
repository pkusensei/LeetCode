mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn last_integer(n: i64) -> i64 {
    dfs(n, n, true)
}

fn dfs(n: i64, count: i64, dir: bool) -> i64 {
    if count > 2 {
        dfs(n, (1 + count) / 2, !dir)
    } else {
        if dir { 1 } else { n }
    }
}

// q2
pub fn maximum_sum(mut nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut rem0 = [0; 2];
    let mut rem1 = [0; 2];
    let mut rem2 = [0; 2];
    for &num in nums.iter() {
        match num % 3 {
            0 => {
                if rem0.iter().all(|&v| v > 0) {
                    res = res.max(num + rem0[0] + rem0[1]);
                }
                if rem1[0] > 0 && rem2[0] > 0 {
                    res = res.max(num + rem1[0] + rem2[0]);
                }
                if num > rem0[0] {
                    rem0[1] = rem0[0];
                    rem0[0] = num;
                } else if num > rem0[1] {
                    rem0[1] = num;
                }
            }
            1 => {
                if rem1.iter().all(|&v| v > 0) {
                    res = res.max(num + rem1[0] + rem1[1]);
                }
                if rem0[0] > 0 && rem2[0] > 0 {
                    res = res.max(num + rem0[0] + rem2[0]);
                }
                if num > rem1[0] {
                    rem1[1] = rem1[0];
                    rem1[0] = num;
                } else if num > rem1[1] {
                    rem1[1] = num;
                }
            }
            2 => {
                if rem2.iter().all(|&v| v > 0) {
                    res = res.max(num + rem2[0] + rem2[1]);
                }
                if rem0[0] > 0 && rem1[0] > 0 {
                    res = res.max(num + rem0[0] + rem1[0]);
                }
                if num > rem2[0] {
                    rem2[1] = rem2[0];
                    rem2[0] = num;
                } else if num > rem2[1] {
                    rem2[1] = num;
                }
            }
            _ => unreachable!(),
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
        // assert_eq!(last_integer(5), 1);
        // assert_eq!(last_integer(9), 9);

        assert_eq!(maximum_sum(vec![4, 2, 3, 1]), 9);
        assert_eq!(maximum_sum(vec![2, 1, 5]), 0);
    }

    #[test]
    fn test() {}
}

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    let mut res = vec![0; 2 * n as usize - 1];
    let mut seen = vec![false; 1 + n as usize];
    backtrack(n, &mut res, &mut seen, 0);
    res
}

fn backtrack(n: i32, res: &mut Vec<i32>, seen: &mut [bool], idx: usize) -> bool {
    if idx >= res.len() {
        return true;
    }
    if res[idx] > 0 {
        backtrack(n, res, seen, 1 + idx)
    } else {
        for num in (1..=n).rev() {
            if seen[num as usize] {
                continue;
            }
            seen[num as usize] = true;
            res[idx] = num;
            if num == 1 {
                if backtrack(n, res, seen, 1 + idx) {
                    return true;
                }
            } else if res.get(idx + num as usize).is_some_and(|&v| v == 0) {
                res[idx + num as usize] = num;
                if backtrack(n, res, seen, 1 + idx) {
                    return true;
                }
                res[idx + num as usize] = 0;
            }
            res[idx] = 0;
            seen[num as usize] = false;
        }
        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(construct_distanced_sequence(5), [5, 3, 1, 4, 3, 5, 2, 4, 2]);
        assert_eq!(construct_distanced_sequence(3), [3, 1, 2, 3, 2]);
    }

    #[test]
    fn test() {}
}

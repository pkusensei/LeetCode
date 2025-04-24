mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn most_frequent_prime(mat: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let [rows, cols] = get_dimensions(&mat);
    let mut map = HashMap::new();
    for r in 0..rows {
        for c in 0..cols {
            for dir in ALL_DIRS {
                for v in build_num(&mat, r as i32, c as i32, dir) {
                    *map.entry(v).or_insert(0) += 1;
                }
            }
        }
    }
    let Some(&max_f) = map.values().max() else {
        return -1;
    };
    map.into_iter()
        .filter_map(|(num, freq)| if freq == max_f { Some(num) } else { None })
        .max()
        .unwrap_or(-1)
}

fn build_num(mat: &[Vec<i32>], mut r: i32, mut c: i32, dir: [i32; 2]) -> Vec<i32> {
    let [rows, cols] = get_dimensions(mat);
    let mut res = vec![];
    let mut curr = 0;
    while r >= 0 && c >= 0 && rows > r as usize && cols > c as usize {
        curr = curr * 10 + mat[r as usize][c as usize];
        r += dir[0];
        c += dir[1];
        if curr > 10 && is_prime(curr) {
            res.push(curr);
        }
    }
    res
}

fn is_prime(num: i32) -> bool {
    if num < 2 {
        return false;
    }
    let root = num.isqrt();
    for p in 2..=root {
        if num % p == 0 {
            return false;
        }
    }
    true
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
    fn basics() {}

    #[test]
    fn test() {}
}

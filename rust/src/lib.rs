mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(points: &[i32], m: i32) -> i64 {
    let mut left = 0;
    let mut right = 10_i64.pow(15);
    while left < right {
        let mid = left + (right - left + 1) / 2;
        if check(points, m.into(), mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
}

fn check(points: &[i32], m: i64, min: i64) -> bool {
    let [mut moves, mut prev] = [0, 0];
    for (idx, num) in points.iter().map(|&v| i64::from(v)).enumerate() {
        // number of increments to reach `min`
        let need = (min + num - 1) / num - prev;
        if need > 0 {
            // back and forth, so that
            // [idx] takes `need` moves
            // [1+idx] takes `need-` moves
            moves += 2 * need - 1;
            prev = need - 1; // moves already spent on [1+idx]
        } else if 1 + idx < points.len() {
            // Current [idx] already satisfied, slide forward
            moves += 1;
            prev = 0;
        }
        if moves > m {
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
    fn basics() {
        assert_eq!(max_score(&[2, 4], 3), 4);
        assert_eq!(max_score(&[1, 2, 3], 5), 2);
    }

    #[test]
    fn test() {}
}

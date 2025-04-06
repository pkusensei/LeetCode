mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_black_blocks(m: i32, n: i32, coordinates: &[[i32; 2]]) -> Vec<i64> {
    use itertools::iproduct;
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for c in coordinates {
        let [x, y] = c[..] else { unreachable!() };
        for (dx, dy) in iproduct!(-1..=0, -1..=0) {
            let bx = x + dx;
            let by = y + dy;
            if (0..m - 1).contains(&bx) && (0..n - 1).contains(&by) {
                *map.entry([bx, by]).or_insert(0) += 1;
            }
        }
    }
    let mut res = vec![];
    let [m, n] = [m, n].map(|v| v as usize - 1);
    res.push((m * n - map.len()) as i64);
    for val in 1..=4 {
        let c = map.values().filter(|&&v| v == val).count();
        res.push(c as i64);
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
        assert_eq!(count_black_blocks(3, 3, &[[0, 0]]), [3, 1, 0, 0, 0]);
        assert_eq!(
            count_black_blocks(3, 3, &[[0, 0], [1, 1], [0, 2]]),
            [0, 2, 2, 0, 0]
        );
    }

    #[test]
    fn test() {}
}

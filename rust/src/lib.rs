mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i64> {
    let [x, y] = [x.min(y) as usize, x.max(y) as usize];
    let n = n as usize;
    let mut diff = vec![0i64; n];
    // This i is the house
    for i in 1..=n {
        // Every i can go left or right
        diff[0] += 2;
        // From here on, i is distance k
        // To go left to 1, take either
        // 1) direct route i-1
        // 2) shortcut i->y->x->1
        //    2.1) Since x<=y, i->x->y->1 is always an longer path
        // Any path longer is invalid; discount it from diff array
        diff[(i - 1).min(i.abs_diff(y) + x)] -= 1;
        // Similarly, go right to n
        diff[(n - i).min(i.abs_diff(x) + n + 1 - y)] -= 1;
        // Possible shortcuts
        diff[i.abs_diff(x).min(i.abs_diff(y) + 1)] += 1;
        diff[(i.abs_diff(x) + 1).min(i.abs_diff(y))] += 1;
        // min dist to x or y
        // it's 0 if x<=i<=y
        let min_dist = x.saturating_sub(i) + i.saturating_sub(y);
        // Reduce 2 counts when crossing x.midpoint(y)
        diff[min_dist + (y - x + 0) / 2] -= 1;
        diff[min_dist + (y - x + 1) / 2] -= 1;
    }
    for i in 1..n {
        diff[i] += diff[i - 1]
    }
    diff
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
        assert_eq!(count_of_pairs(3, 1, 3), [6, 0, 0]);
        assert_eq!(count_of_pairs(5, 2, 4), [10, 8, 2, 0, 0]);
        assert_eq!(count_of_pairs(4, 1, 1), [6, 4, 2, 0]);
    }

    #[test]
    fn test() {}
}

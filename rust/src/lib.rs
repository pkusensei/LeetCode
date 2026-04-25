mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_distance(side: i32, points: &[[i32; 2]], k: i32) -> i32 {
    let side = i64::from(side);
    let mut arr = vec![];
    for p in points.iter() {
        let [x, y] = [0, 1].map(|i| i64::from(p[i]));
        if x == 0 {
            arr.push(y); // left
        } else if y == side {
            arr.push(side + x); // top
        } else if x == side {
            arr.push(3 * side - y); // right
        } else {
            arr.push(4 * side - x); // bottom
        }
    }
    arr.sort_unstable();
    let mut left = 0;
    let mut right = side;
    while left < right {
        let mid = left + (right - left + 1) / 2;
        if f(&arr, side, k, mid) {
            left = mid
        } else {
            right = mid - 1
        }
    }
    left as i32
}

fn f(arr: &[i64], side: i64, k: i32, mid: i64) -> bool {
    let total = 4 * side;
    'out: for &start in arr {
        // For each start, this is the farthest point on "1D" arr
        let end = total + start - mid;
        let mut curr = start;
        for _ in 1..k {
            let i = arr.partition_point(|&v| v < curr + mid);
            if arr.get(i).is_none_or(|&v| v > end) {
                continue 'out; // Impossible `start`; try next one
            }
            curr = arr[i];
        }
        return true;
    }
    false
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
    fn basics() {}

    #[test]
    fn test() {}
}

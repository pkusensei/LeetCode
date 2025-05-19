mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_path_length(coordinates: &[[i32; 2]], k: i32) -> i32 {
    let [kx, ky] = [0, 1].map(|i| coordinates[k as usize][i]);
    let [mut left, mut right] = [vec![], vec![]];
    for c in coordinates.iter() {
        let [x, y] = c[..] else { unreachable!() };
        if x < kx && y < ky {
            left.push([x, y]);
        } else if x > kx && y > ky {
            right.push([x, y]);
        }
    }
    let func = |a: &[i32; 2], b: &[i32; 2]| a[0].cmp(&b[0]).then(b[1].cmp(&a[1]));
    left.sort_unstable_by(func);
    right.sort_unstable_by(func);
    (lis(&left) + lis(&right) + 1) as i32
}

fn lis(nums: &[[i32; 2]]) -> usize {
    let mut arr = vec![];
    for &[_, y] in nums {
        let i = arr.partition_point(|&v| v < y);
        if i == arr.len() {
            arr.push(y);
        } else {
            arr[i] = y; // useless?
        }
    }
    arr.len()
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
        assert_eq!(
            max_path_length(&[[3, 1], [2, 2], [4, 1], [0, 0], [5, 3]], 1),
            3
        );
        assert_eq!(max_path_length(&[[2, 1], [7, 0], [5, 6]], 2), 2);
    }

    #[test]
    fn test() {
        assert_eq!(max_path_length(&[[2, 1], [5, 4], [9, 8]], 0), 3);
    }
}

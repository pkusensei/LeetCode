mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let k = k as usize;
    let [rows, cols] = get_dimensions(&matrix);
    let mut nums = Vec::with_capacity(rows * cols);
    let mut prev = vec![];
    for row in matrix.iter() {
        let mut curr = row.iter().fold(vec![], |mut acc, num| {
            acc.push(num ^ acc.last().unwrap_or(&0));
            acc
        });
        for (i, v) in curr.iter_mut().enumerate() {
            *v = (*v) ^ prev.get(i).unwrap_or(&0);
        }
        nums.extend_from_slice(&curr);
        prev = curr;
    }
    let (_, v, _) = nums.select_nth_unstable(rows * cols - k);
    *v
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
        assert_eq!(kth_largest_value(vec![vec![5, 2], vec![1, 6]], 1), 7);
        assert_eq!(kth_largest_value(vec![vec![5, 2], vec![1, 6]], 2), 5);
        assert_eq!(kth_largest_value(vec![vec![5, 2], vec![1, 6]], 3), 4);
    }

    #[test]
    fn test() {}
}

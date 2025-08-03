mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_total_fruits(fruits: &[[i32; 2]], start_pos: i32, k: i32) -> i32 {
    let i = fruits.partition_point(|f| f[0] < start_pos - k);
    let [mut left, mut right] = [i, i];
    let mut res = 0;
    let mut curr = 0;
    while let Some(left_f) = fruits.get(left) {
        let left_pos = left_f[0].min(start_pos);
        let left_steps = start_pos - left_pos;
        // walk left first:  2*left_steps + right_steps = k
        // walk right first: 2*right_steps + left_steps = k
        let right_steps = (k - 2 * left_steps).max((k - left_steps) / 2);
        let right_pos = start_pos + right_steps;
        while let Some(f) = fruits.get(right)
            && f[0] <= right_pos
        {
            curr += f[1];
            right += 1;
        }
        res = res.max(curr);
        curr -= left_f[1];
        if left_f[0] > start_pos {
            // [left] is already to the right of start
            // i.e goes right straight
            // Now, what happens if this is the first [left] in loop?
            // Left point is computed by min([left], start)
            // When start<[left] it is start => goes right straight
            break;
        }
        left += 1;
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
        assert_eq!(max_total_fruits(&[[2, 8], [6, 3], [8, 6]], 5, 4), 9);
        assert_eq!(
            max_total_fruits(&[[0, 9], [4, 1], [5, 7], [6, 2], [7, 4], [10, 9]], 5, 4),
            14
        );
        assert_eq!(max_total_fruits(&[[0, 3], [6, 4], [8, 5]], 3, 2), 0);
    }

    #[test]
    fn test() {}
}

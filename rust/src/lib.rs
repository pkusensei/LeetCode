mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_operations(grid: &[&[i32]]) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    // min op counts per column
    let mut freq = vec![[rows; 10]; cols];
    for row in grid.iter() {
        for (c, &v) in row.iter().enumerate() {
            freq[c][v as usize] -= 1;
        }
    }
    let mut dp = freq[0];
    for col in &freq[1..] {
        let mut curr = [1_000_000; 10];
        for curr_v in 0..10 {
            for prev in 0..10 {
                if prev != curr_v {
                    curr[curr_v] = curr[curr_v].min(col[curr_v] + dp[prev])
                }
            }
        }
        dp = curr;
    }
    dp.into_iter().min().unwrap() as i32
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
        assert_eq!(minimum_operations(&[&[1, 1, 1], &[0, 0, 0]]), 3);
        assert_eq!(minimum_operations(&[&[1, 0, 2], &[1, 0, 2]]), 0);
        assert_eq!(minimum_operations(&[&[1], &[2], &[3]]), 2);
    }

    #[test]
    fn test() {}
}

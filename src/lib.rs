mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn stone_game_ii(piles: &[i32]) -> i32 {
    let size = piles.len();
    if size <= 2 {
        return piles.iter().sum();
    }

    let mut suffix_sum = piles.to_vec();
    for i in (0..size - 1).rev() {
        suffix_sum[i] += suffix_sum[i + 1];
    }
    let mut memo = vec![vec![0; size]; size];
    max_stones(&suffix_sum, 1, 0, &mut memo)
}

fn max_stones(suffix_sum: &[i32], curr_max: usize, curr_idx: usize, memo: &mut [Vec<i32>]) -> i32 {
    if curr_idx + 2 * curr_max >= suffix_sum.len() {
        return suffix_sum[curr_idx];
    }
    if memo[curr_idx][curr_max] > 0 {
        return memo[curr_idx][curr_max];
    }

    let mut res = i32::MAX;
    for i in 1..=2 * curr_max {
        res = res.min(max_stones(suffix_sum, curr_max.max(i), curr_idx + i, memo));
    }
    memo[curr_idx][curr_max] = suffix_sum[curr_idx] - res;
    memo[curr_idx][curr_max]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(stone_game_ii(&[2, 7, 9, 4, 4]), 10);
        debug_assert_eq!(stone_game_ii(&[1, 2, 3, 4, 5, 100]), 104);
    }

    #[test]
    fn test() {}
}

mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_rotate_steps(ring: &str, key: &str) -> i32 {
    dfs(
        ring.as_bytes(),
        key.as_bytes(),
        0,
        0,
        &mut vec![vec![0; key.len()]; ring.len()],
    )
}

fn dfs(ring: &[u8], key: &[u8], r_idx: usize, k_idx: usize, dp: &mut [Vec<i32>]) -> i32 {
    if k_idx == key.len() {
        return 0;
    }
    if dp[r_idx][k_idx] > 0 {
        return dp[r_idx][k_idx];
    }
    if ring[r_idx] == key[k_idx] {
        // dp is faster when more results cached
        // when more smaller subproblems?
        let res = 1 + dfs(ring, key, r_idx, k_idx + 1, dp);
        dp[r_idx][k_idx] = res;
        return res;
    }

    let (mut right, mut r_count) = (r_idx, 0);
    while right < ring.len() && ring[right] != key[k_idx] {
        r_count += 1;
        right = (right + 1) % ring.len();
    }
    let (mut left, mut l_count) = (r_idx, 0);
    while ring[left] != key[k_idx] {
        l_count += 1;
        left = left.checked_sub(1).unwrap_or(ring.len() - 1);
    }

    let res =
        (r_count + dfs(ring, key, right, k_idx, dp)).min(l_count + dfs(ring, key, left, k_idx, dp));
    dp[r_idx][k_idx] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_rotate_steps("godding", "gd"), 4);
        debug_assert_eq!(find_rotate_steps("godding", "godding"), 13);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}

mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_max_sums(mut nums: Vec<i32>, k: i32) -> i32 {
    const M: usize = 1_000_000_007;
    let n = nums.len();
    let k = k as usize;
    let mut comb = vec![vec![0; 1 + k]; 1 + n];
    comb[0][0] = 1;
    for row in 1..=n {
        comb[row][0] = 1;
        for col in 1..=k {
            comb[row][col] = (comb[row - 1][col - 1] + comb[row - 1][col]) % M;
        }
    }
    nums.sort_unstable();
    let mut res = 0;
    for i in 0..n {
        let mut subseq = 0;
        for pick in 0..k {
            if pick <= i {
                subseq = (subseq + comb[i][pick]) % M;
            }
        }
        res += (nums[i] as usize + nums[n - 1 - i] as usize) * subseq % M;
        res %= M;
    }
    res as i32
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
        assert_eq!(min_max_sums(vec![1, 2, 3], 2), 24);
    }

    #[test]
    fn test() {}
}

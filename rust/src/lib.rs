mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximize_xor_and_xor(nums: &[i32]) -> i64 {
    let n = nums.len();
    let total_xor = nums.iter().fold(0, |acc, v| acc ^ v);
    let mut res = 0;
    for mask in 0..1 << n {
        let mut select_and = i32::MAX;
        let mut select_xor = 0;
        for (i, &num) in nums.iter().enumerate() {
            if (mask >> i) & 1 == 1 {
                select_and &= num;
                select_xor ^= num;
            }
        }
        if select_and == i32::MAX {
            select_and = 0;
        }
        // With fixed select_and, we want max of
        // x + unselect_xor ^ x == unselect_xor + 2*(x & inverted)
        let unselect_xor = total_xor ^ select_xor;
        // bit_not, i.e 0=>1 1=>0
        // These are the bits that could be used
        let inverted = !unselect_xor;
        // Linear independence
        // i.e for any a, b, c in basis, a^b != c
        // Here it records all bit patterns of (x&inverted)
        let mut basis = vec![];
        for (i, &num) in nums.iter().enumerate() {
            // For each number not in select_and group
            if (mask >> i) & 1 == 0 {
                // Simplify its bit form
                let mut reduced = num & inverted;
                // And further reduce any bit already recorded in basis
                for b in &basis {
                    reduced = reduced.min(reduced ^ b);
                }
                if reduced > 0 {
                    basis.push(reduced);
                }
            }
        }
        // To find max(x&inverted)
        // Greedily pick the max xor combination of bit patterns
        let mut max_xor = 0;
        for b in &basis {
            max_xor = max_xor.max(max_xor ^ b);
        }
        let curr = i64::from(select_and) + i64::from(unselect_xor) + 2 * i64::from(max_xor);
        res = res.max(curr);
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
        assert_eq!(maximize_xor_and_xor(&[2, 3]), 5);
        assert_eq!(maximize_xor_and_xor(&[1, 3, 2]), 6);
        assert_eq!(maximize_xor_and_xor(&[2, 3, 6, 7]), 15);
    }

    #[test]
    fn test() {}
}

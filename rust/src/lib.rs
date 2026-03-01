mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_or(grid: &[&[i32]]) -> i32 {
    let mut res = 0;
    // Greedily ignore these bits
    let mut discard = 0;
    'out: for bit in (0..=17).rev() {
        // Attempt to turn this bit off
        let temp = discard | (1 << bit);
        for row in grid.iter() {
            // Test against all previous ignored bits
            if row.iter().all(|&v| v & temp != 0) {
                // Fail to turn it of
                // Record it in `res`
                res |= 1 << bit;
                continue 'out;
            }
        }
        // Turn this `bit` off
        discard |= 1 << bit;
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
        assert_eq!(minimum_or(&[&[1, 5], &[2, 4]]), 3);
        assert_eq!(minimum_or(&[&[3, 5], &[6, 4]]), 5);
        assert_eq!(minimum_or(&[&[7, 9, 8]]), 7);
    }

    #[test]
    fn test() {
        assert_eq!(minimum_or(&[&[14, 7]]), 7);
    }
}

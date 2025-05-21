mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_bitwise_array(nums: &[i32]) -> Vec<i32> {
    let mut res = vec![];
    for &num in nums.iter() {
        if num == 2 {
            res.push(-1);
        } else if (1 + num).count_ones() == 1 {
            res.push(num >> 1); // all 1's
        } else {
            // 23 => 0b10111
            // find 0b111 => shift right by 1 => 0b11
            // 0b11 | (1+ 0b11) = 0b111
            // bitor it back onto 0b10000
            let mut val = num;
            let mut count = 0;
            while val & 1 == 1 {
                count += 1;
                val >>= 1;
            }
            let lower_bits = ((1 << count) - 1) >> 1;
            res.push(((num >> count) << count) | lower_bits);
        }
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
        assert_eq!(min_bitwise_array(&[2, 3, 5, 7]), [-1, 1, 4, 3]);
    }

    #[test]
    fn test() {}
}

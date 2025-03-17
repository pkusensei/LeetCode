mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn k_sum(nums: &[i32], k: i32) -> i64 {
    use std::collections::BinaryHeap;
    let mut vals = Vec::with_capacity(nums.len());
    let mut res = 0;
    for &num in nums.iter() {
        let v = i64::from(num);
        res += v.max(0);
        vals.push(v.abs());
    }
    vals.sort_unstable();
    // First, try max_sum-vals[0]
    let mut heap = BinaryHeap::from([(res - vals[0], 0)]);
    // res starts with max sum => 1..k
    for _ in 1..k {
        let Some((sum, idx)) = heap.pop() else {
            break;
        };
        res = sum;
        if let Some(next) = vals.get(1 + idx) {
            heap.push((sum - next, 1 + idx));
            heap.push((sum + vals[idx] - next, 1 + idx));
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
        assert_eq!(k_sum(&[2, 4, -2], 5), 2);
        assert_eq!(k_sum(&[1, -2, 3, 4, -10, 12], 16), 10);
    }

    #[test]
    fn test() {}
}

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
    let mut heap = std::collections::BinaryHeap::from([a, b, c]);
    let mut res = 0;
    while heap.len() > 1 {
        res += 1;
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        if a > 1 {
            heap.push(a - 1);
        }
        if b > 1 {
            heap.push(b - 1);
        }
    }
    res
}

pub fn with_sort(a: i32, b: i32, c: i32) -> i32 {
    let mut nums = [a, b, c];
    nums.sort_unstable();
    let val = nums[0] + nums[1];
    // Suppose after sorting a<b<c
    if val < nums[2] {
        // a+b<c
        // Each turn take one from c and one from a or b
        // It takes (a+b) turns
        val
    } else {
        // c<a+b
        // Keep taking from thr two bigger piles
        // Reduce them all down to 0 or 1
        // And reduce their diff to at most 1
        (val + nums[2]) / 2
    }
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
    fn basics() {}

    #[test]
    fn test() {}
}

mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_pair_removal(nums: &[i32]) -> i32 {
    use std::collections::BTreeSet;
    let n = nums.len();
    let mut arr: Vec<_> = nums.iter().map(|&v| i64::from(v)).collect();
    let mut set = BTreeSet::new();
    let mut prev = vec![None; n];
    let mut next = vec![n; n];
    let mut unruly = 0;
    for (i, w) in nums.windows(2).enumerate() {
        unruly += i32::from(w[0] > w[1]);
        let v = i64::from(w[0] + w[1]);
        set.insert((v, i));
        prev[1 + i] = Some(i);
        next[i] = 1 + i;
    }
    let mut res = 0;
    while unruly > 0 {
        let Some((val, idx)) = set.pop_first() else {
            break;
        };
        // left, idx, right, rright
        let left = prev[idx];
        let right = next[idx];
        let rright = next[right];
        unruly -= i32::from(arr[idx] > arr[right]);
        if let Some(left) = left {
            if arr[left] > arr[idx] && arr[left] <= val {
                unruly -= 1;
            } else if arr[left] <= arr[idx] && arr[left] > val {
                unruly += 1;
            }
            set.remove(&(arr[left] + arr[idx], left));
            set.insert((arr[left] + val, left));
        }
        if rright < n {
            if arr[rright] >= arr[right] && arr[rright] < val {
                unruly += 1;
            } else if arr[rright] < arr[right] && arr[rright] >= val {
                unruly -= 1;
            }
            set.remove(&(arr[right] + arr[rright], right));
            set.insert((val + arr[rright], idx));
            prev[rright] = Some(idx);
        }
        next[idx] = rright;
        arr[idx] = val;
        res += 1;
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
        assert_eq!(minimum_pair_removal(&[5, 2, 3, 1]), 2);
        assert_eq!(minimum_pair_removal(&[1, 2, 2]), 0);
    }

    #[test]
    fn test() {}
}

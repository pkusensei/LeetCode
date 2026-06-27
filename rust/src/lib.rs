mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn duplicate_zeros(arr: &mut [i32]) {
    let mut right = arr.len() - 1;
    let mut dups = 0;
    let mut left = 0;
    while left <= right - dups {
        // First iter forwards to count zeros
        // Until 1 past what would be last element of "new" array
        if arr[left] == 0 {
            // This zero stands on the end of "new" array
            // Its dup has no place to go
            // Thus we copy this zero directly w/o counting it
            if left == right - dups {
                arr[right] = 0;
                right -= 1;
                break;
            }
            dups += 1;
        }
        left += 1;
    }
    for i in (0..=right - dups).rev() {
        if arr[i] == 0 {
            arr[i + dups] = 0;
            dups -= 1;
            arr[i + dups] = 0;
        } else {
            arr[i + dups] = arr[i]
        }
    }
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
    fn basics() {}

    #[test]
    fn test() {}
}

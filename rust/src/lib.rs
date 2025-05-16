mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_k_constraint_substrings(s: &str, k: i32, queries: &[[i32; 2]]) -> Vec<i64> {
    let (s, n) = (s.as_bytes(), s.len());
    let mut left = 0;
    let mut count = [0, 0];
    let mut left_right = vec![n; n];
    let mut prefix = vec![];
    for (right, &b) in s.iter().enumerate() {
        count[usize::from(b - b'0')] += 1;
        while count[0] > k && count[1] > k {
            // first invalid idx for left
            // i.e half open [left, right)
            left_right[left] = right;
            count[usize::from(s[left] - b'0')] -= 1;
            left += 1;
        }
        // right-left+1 = count of valid substring
        // prefix[right+1] = prefix sum of all substrs up until right
        prefix.push(right - left + 1 + prefix.last().unwrap_or(&0));
    }
    let mut res = vec![];
    for q in queries.iter() {
        let [left, right] = [0, 1].map(|i| q[i] as usize);
        let valid_from_left = (1 + right).min(left_right[left]);
        // All substrs in [left, valid_from_left)
        let left_count = (1 + valid_from_left - left) * (valid_from_left - left) / 2;
        // All substrs in [right_end, right]
        let right_count = prefix[right]
            - if valid_from_left > 0 {
                prefix[valid_from_left - 1]
            } else {
                0
            };
        res.push((left_count + right_count) as i64);
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
        assert_eq!(count_k_constraint_substrings("0001111", 2, &[[0, 6]]), [26]);
        assert_eq!(
            count_k_constraint_substrings("010101", 1, &[[0, 5], [1, 4], [2, 3]]),
            [15, 9, 3]
        );
    }

    #[test]
    fn test() {}
}

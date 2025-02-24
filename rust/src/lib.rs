mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn plates_between_candles(s: &str, queries: &[[i32; 2]]) -> Vec<i32> {
    let (s, n) = (s.as_bytes(), s.len());
    let [mut left_indices, mut right_indices] = [0, 1].map(|_| Vec::with_capacity(n));
    let mut prefix = Vec::with_capacity(n);
    let mut candle = -1;
    for (i, &b) in s.iter().enumerate() {
        prefix.push(i32::from(b == b'|') + prefix.last().unwrap_or(&0));
        if b == b'|' {
            candle = i as i32;
        }
        left_indices.push(candle);
    }
    candle = -1;
    for (i, &b) in s.iter().enumerate().rev() {
        if b == b'|' {
            candle = i as i32;
        }
        right_indices.push(candle);
    }
    right_indices.reverse();
    let mut res = vec![];
    for q in queries.iter() {
        let left = right_indices[q[0] as usize]; // left boundary
        let right = left_indices[q[1] as usize]; // right boundary
        if right == -1 || left == -1 || right <= left {
            res.push(0);
        } else {
            let count = prefix[right as usize]
                - if left > 0 {
                    prefix[left as usize - 1]
                } else {
                    0
                };
            res.push(right + 1 - left - count);
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
        assert_eq!(
            plates_between_candles("**|**|***|", &[[2, 5], [5, 9]]),
            [2, 3]
        );
        assert_eq!(
            plates_between_candles(
                "***|**|*****|**||**|*",
                &[[1, 17], [4, 5], [14, 17], [5, 11], [15, 16]]
            ),
            [9, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test() {}
}

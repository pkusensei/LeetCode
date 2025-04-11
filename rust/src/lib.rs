mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_max_function_value(receiver: &[i32], k: i64) -> i64 {
    let n = receiver.len();
    let k = k as usize;
    let mut edges = vec![];
    // [node, score]
    let mut prev: Vec<_> = receiver.iter().map(|&v| [v as usize; 2]).collect();
    let mut dist = 2;
    while dist <= k {
        let next = prev
            .iter()
            .map(|&[node, score]| {
                let [next, next_score] = prev[node];
                [next, score + next_score]
            })
            .collect();
        edges.push(prev);
        prev = next;
        dist <<= 1;
    }
    edges.push(prev);
    let mut res = 0;
    for mut node in 0..n {
        let mut score = node;
        let mut b = 0;
        while (1 << b) <= k {
            if (1 << b) & k > 0 {
                let [next, s] = edges[b][node];
                node = next;
                score += s;
            }
            b += 1;
        }
        res = res.max(score)
    }
    res as i64
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
        assert_eq!(get_max_function_value(&[2, 0, 1], 4), 6);
        assert_eq!(get_max_function_value(&[1, 1, 1, 2, 3], 3), 10);
    }

    #[test]
    fn test() {}
}

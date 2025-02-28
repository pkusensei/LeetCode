mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
    let n = bombs.len();
    let mut adj = vec![vec![]; n];
    for (i1, b1) in bombs.iter().enumerate() {
        let [x1, y1, r1] = [0, 1, 2].map(|i| i64::from(b1[i]));
        for (i2, b2) in bombs.iter().enumerate() {
            if i1 == i2 {
                continue;
            }
            let [x2, y2] = [0, 1].map(|i| i64::from(b2[i]));
            if (x1 - x2).pow(2) + (y1 - y2).pow(2) <= r1.pow(2) {
                adj[i1].push(i2);
            }
        }
    }
    let mut res = 0;
    for start in 0..n {
        let mut queue = std::collections::VecDeque::from([start]);
        let mut seen = vec![false; n];
        seen[start] = true;
        let mut curr = 0;
        while let Some(node) = queue.pop_front() {
            curr += 1;
            for &next in adj[node].iter() {
                if !seen[next] {
                    seen[next] = true;
                    queue.push_back(next);
                }
            }
        }
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
    fn basics() {}

    #[test]
    fn test() {}
}

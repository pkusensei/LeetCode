mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn query_results(_limit: i32, queries: &[[i32; 2]]) -> Vec<i32> {
    let mut ball_color: HashMap<i32, i32> = HashMap::new();
    let mut color_count: HashMap<i32, i32> = HashMap::new();
    let mut res = vec![];
    for q in queries.iter() {
        let [ball, color] = q[..] else { unreachable!() };
        if let Some(c) = ball_color.get_mut(&ball) {
            color_count.entry(*c).and_modify(|v| *v -= 1);
            if color_count[c] == 0 {
                color_count.remove(c);
            }
            *c = color;
        } else {
            ball_color.insert(ball, color);
        }
        *color_count.entry(color).or_insert(0) += 1;
        res.push(color_count.len() as i32);
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
            query_results(4, &[[1, 4], [2, 5], [1, 3], [3, 4]]),
            [1, 2, 2, 3]
        );
        assert_eq!(
            query_results(4, &[[0, 1], [1, 2], [2, 2], [3, 4], [4, 5]]),
            [1, 2, 2, 3, 4]
        );
    }

    #[test]
    fn test() {}
}

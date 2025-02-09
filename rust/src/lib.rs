mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_building(n: i32, mut rests: Vec<[i32; 2]>) -> i32 {
    rests.extend([[1, 0], [n, n - 1]]);
    rests.sort_unstable();
    let len = rests.len();
    for idx in 0..len - 1 {
        let dist = rests[idx + 1][0] - rests[idx][0];
        rests[idx + 1][1] = rests[idx + 1][1].min(rests[idx][1] + dist);
    }
    for idx in (0..len - 1).rev() {
        let dist = rests[idx + 1][0] - rests[idx][0];
        rests[idx][1] = rests[idx][1].min(rests[idx + 1][1] + dist);
    }
    let mut res = 0;
    for idx in 0..len - 1 {
        let dist = rests[idx + 1][0] - rests[idx][0];
        let h1 = rests[idx][1];
        let h2 = rests[idx + 1][1];
        res = res.max(h1.max(h2) + (dist - (h1 - h2).abs()) / 2);
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
        assert_eq!(max_building(5, vec![[2, 1], [4, 1]]), 2);
        assert_eq!(max_building(6, vec![]), 5);
        assert_eq!(max_building(10, vec![[5, 3], [2, 5], [7, 4], [10, 3]]), 5);
    }

    #[test]
    fn test() {}
}

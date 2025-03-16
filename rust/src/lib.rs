mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_cycle(edges: &[i32]) -> i32 {
    let n = edges.len();
    let mut res = -1;
    let mut ids = vec![-1; n];
    let mut id = 0;
    for i in 0..n as i32 {
        let mut node = i;
        let curr = id;
        while node != -1 && ids[node as usize] == -1 {
            ids[node as usize] = id;
            id += 1;
            node = edges[node as usize];
        }
        if node != -1 && ids[node as usize] >= curr {
            res = res.max(id - ids[node as usize]);
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
        assert_eq!(longest_cycle(&[3, 3, 4, 2, 3]), 3);
        assert_eq!(longest_cycle(&[2, -1, 3, 1]), -1);
    }

    #[test]
    fn test() {}
}

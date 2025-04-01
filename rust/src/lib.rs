mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn color_the_array(n: i32, queries: &[[i32; 2]]) -> Vec<i32> {
    let n = n as usize;
    let mut v = vec![0; n];
    let mut res = vec![];
    let mut curr = 0;
    for q in queries {
        let idx = q[0] as usize;
        let color = q[1];
        if 1 + idx < n {
            if v[1 + idx] == v[idx] && v[idx] > 0 {
                curr -= 1;
            }
            if v[1 + idx] == color {
                curr += 1;
            }
        }
        if idx > 0 {
            if v[idx - 1] == v[idx] && v[idx] > 0 {
                curr -= 1;
            }
            if v[idx - 1] == color {
                curr += 1;
            }
        }
        v[idx] = color;
        res.push(curr);
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
            color_the_array(4, &[[0, 2], [1, 2], [3, 1], [1, 1], [2, 1]]),
            [0, 1, 1, 0, 2]
        );
        assert_eq!(color_the_array(1, &[[0, 100000]]), [0]);
    }

    #[test]
    fn test() {}
}

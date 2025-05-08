mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
    let n = colors.len();
    let mut res = 0;
    for i in 0..n {
        let left = i.checked_sub(1).unwrap_or(n - 1);
        let right = (1 + i) % n;
        res += i32::from(colors[left] != colors[i] && colors[i] != colors[right]);
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
        assert_eq!(number_of_alternating_groups(vec![0, 1, 0, 0, 1]), 3);
    }

    #[test]
    fn test() {}
}

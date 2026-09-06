mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_groups(position: &[i32], speed: &[i32], distance: i32) -> i32 {
    let mut st1: Vec<(i32, i32)> = vec![];
    for (&pos, &spd) in position.iter().zip(speed.iter()) {
        while st1.last().is_some_and(|v| v.0 + distance >= pos) {
            st1.pop();
        }
        st1.push((pos, spd));
    }
    let mut st2 = vec![];
    for &(_, spd) in st1.iter() {
        while st2.last().is_some_and(|&v| v > spd) {
            st2.pop();
        }
        st2.push(spd);
    }
    st2.len() as i32
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
    fn basics() {
        assert_eq!(count_groups(&[657, 686], &[139, 284], 77), 1);
    }

    #[test]
    fn test() {}
}

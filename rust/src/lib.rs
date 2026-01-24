mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let mut res = seats.iter().take_while(|&&v| v == 0).count();
    res = res.max(seats.iter().rev().take_while(|&&v| v == 0).count());
    seats
        .chunk_by(|a, b| a == b)
        .filter_map(|ch| if ch[0] == 0 { Some(ch.len()) } else { None })
        .map(|v| (1 + v) / 2)
        .max()
        .unwrap_or_default()
        .max(res) as i32
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
    fn basics() {}

    #[test]
    fn test() {}
}

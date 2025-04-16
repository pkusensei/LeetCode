mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut it = edges
        .iter()
        .fold(vec![0; n as usize], |mut acc, e| {
            acc[e[1] as usize] += 1;
            acc
        })
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| if v == 0 { Some(i) } else { None });
    match (it.next(), it.next()) {
        (Some(i), None) => i as i32,
        _ => -1,
    }
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

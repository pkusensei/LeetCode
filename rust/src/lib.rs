mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn winner_of_game(colors: &str) -> bool {
    colors
        .as_bytes()
        .chunk_by(|a, b| a == b)
        .filter_map(|ch| {
            let n = ch.len() as i32;
            if n >= 3 {
                Some(if ch[0] == b'A' { n - 2 } else { 2 - n })
            } else {
                None
            }
        })
        .sum::<i32>()
        > 0
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

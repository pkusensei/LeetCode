mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut prefix = vec![0; 1 + right as usize];
    for v in ranges.iter() {
        prefix[v[0] as usize] += 1;
        prefix[1 + v[1] as usize] -= 1;
    }
    for i in 1..=right as usize {
        prefix[i] += prefix[i - 1];
    }
    (left..=right).all(|v| prefix[v as usize] > 0)
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

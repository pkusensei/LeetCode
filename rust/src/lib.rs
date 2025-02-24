mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    tickets[..=k]
        .iter()
        .map(|&v| v.min(tickets[k]))
        .sum::<i32>()
        + tickets[1 + k..]
            .iter()
            .map(|&v| v.min(tickets[k] - 1))
            .sum::<i32>()
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

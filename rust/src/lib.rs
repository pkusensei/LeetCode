mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_days_together(
    arrive_alice: &str,
    leave_alice: &str,
    arrive_bob: &str,
    leave_bob: &str,
) -> i32 {
    let [a1, a2] = [&arrive_alice, &leave_alice].map(|s| parse(s));
    let [b1, b2] = [&arrive_bob, &leave_bob].map(|s| parse(s));
    dbg!(a1, a2);
    dbg!(b1, b2);
    (a2.min(b2) - a1.max(b1) + 1).max(0)
}

fn parse(s: &str) -> i32 {
    const M: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let (a, b) = s.split_once('-').unwrap();
    let mon = a.parse::<usize>().map(|v| v).unwrap_or(1);
    let day = b.parse().unwrap_or(1);
    M[..mon - 1].iter().sum::<i32>() + day
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
        assert_eq!(count_days_together("10-01", "10-31", "11-01", "12-31"), 0);
    }

    #[test]
    fn test() {}
}

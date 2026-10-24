mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
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
    (1 + a2.min(b2) - a1.max(b1)).max(0)
}

fn parse(s: &str) -> i32 {
    const D: [i32; 13] = {
        let mut d = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut i = 1;
        while i <= 12 {
            d[i] += d[i - 1];
            i += 1;
        }
        d
    };
    let Some((m, d)) = s.split_once('-') else {
        return 0;
    };
    let [m, d] = [m, d].map(|s| s.parse::<usize>().unwrap());
    D[m - 1] + d as i32
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
        assert_eq!(count_days_together("10-01", "10-31", "11-01", "12-31"), 0);
    }

    #[test]
    fn test() {}
}

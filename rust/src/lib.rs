mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn convert_time(current: &str, correct: &str) -> i32 {
    let [a, b] = [&current, &correct].map(|s| {
        let (h, m) = s.split_once(':').unwrap();
        60 * h.parse::<i32>().unwrap() + m.parse::<i32>().unwrap()
    });
    let mut diff = b - a;
    let mut res = 0;
    for div in [60, 15, 5, 1] {
        if diff >= div {
            res += diff / div;
            diff %= div;
        }
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
        assert_eq!(convert_time("02:30", "04:35"), 3);
    }

    #[test]
    fn test() {}
}

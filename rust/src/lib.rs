mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_rounds(login_time: &str, logout_time: &str) -> i32 {
    let [mut h1, mut m1] = parse(login_time);
    let [mut h2, mut m2] = parse(logout_time);
    if h2 * 60 + m2 < h1 * 60 + m1 {
        h2 += 24;
    }
    if m1 % 15 > 0 {
        m1 = (m1 / 15 + 1) * 15;
        if m1 == 60 {
            h1 += 1;
            m1 = 0;
        }
    }
    m1 += h1 * 60;
    m2 -= m2 % 15;
    m2 += 60 * h2;
    (m2 - m1).max(0) / 15
}

fn parse(s: &str) -> [i32; 2] {
    let t = s.split_once(':').unwrap();
    let h = t.0.parse().unwrap();
    let m = t.1.parse().unwrap();
    [h, m]
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
        assert_eq!(number_of_rounds("09:31", "10:14"), 1);
        assert_eq!(number_of_rounds("21:30", "03:00"), 22);
    }

    #[test]
    fn test() {}
}

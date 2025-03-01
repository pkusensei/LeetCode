mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn abbreviate_product(left: i32, right: i32) -> String {
    const MAX_SUF: i64 = 100_000_000_000;
    let mut pre = 1.0;
    let mut suf = 1;
    let mut c = 0;
    let mut len = 0;
    for num in left..=right {
        pre *= f64::from(num);
        suf *= i64::from(num);
        while pre >= 100_000.0 {
            pre /= 10.0;
            len = if len == 0 { 6 } else { 1 + len };
        }
        while suf % 10 == 0 {
            suf /= 10;
            c += 1;
        }
        suf %= MAX_SUF;
    }
    let s = suf.to_string();
    format!(
        "{}{}{}e{}",
        pre as i32,
        if len - c <= 10 { "" } else { "..." },
        if len - c < 5 {
            ""
        } else {
            &s[s.len() - (len - c - 5).min(5) as usize..]
        },
        c
    )
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
        assert_eq!(abbreviate_product(1, 4), "24e0");
        assert_eq!(abbreviate_product(2, 11), "399168e2");
        assert_eq!(abbreviate_product(371, 375), "7219856259e3");
    }

    #[test]
    fn test() {}
}

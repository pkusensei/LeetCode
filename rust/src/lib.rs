mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn split_message(message: &str, limit: i32) -> Vec<String> {
    let mut curr = 0;
    let mut b = 0;
    while check(message.len() as _, limit as _, b, curr) {
        b += 1;
        curr += 1 + b.ilog10(); // length of all indices, i.e all a's
    }
    let mut res = vec![];
    if b > 0 && 3 + 2 * (1 + b.ilog10()) < limit as u32 {
        let n = message.len();
        let mut idx = 0;
        for a in 1..=b {
            let len = limit as usize - (1 + a.ilog10()) as usize - 3 - (1 + b.ilog10()) as usize;
            res.push(format!(
                "{}<{}/{}>",
                &message[idx..(idx + len).min(n)],
                a,
                b
            ));
            idx += len;
        }
    }
    res
}

const fn check(len: u32, limit: u32, b: u32, curr: u32) -> bool {
    let width = if b == 0 { 1 } else { 1 + b.ilog10() };
    // len("<b/b>") >= limit => not possible
    // len + len(all a's) + b*len("</b>") > limit*b => b is too small
    3 + width * 2 < limit && curr + len + b * (3 + width) > limit * b
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
        assert_eq!(
            split_message("this is really a very awesome message", 9),
            [
                "thi<1/14>",
                "s i<2/14>",
                "s r<3/14>",
                "eal<4/14>",
                "ly <5/14>",
                "a v<6/14>",
                "ery<7/14>",
                " aw<8/14>",
                "eso<9/14>",
                "me<10/14>",
                " m<11/14>",
                "es<12/14>",
                "sa<13/14>",
                "ge<14/14>"
            ]
        );
        assert_eq!(
            split_message("short message", 15),
            ["short mess<1/2>", "age<2/2>"]
        );
    }

    #[test]
    fn test() {}
}

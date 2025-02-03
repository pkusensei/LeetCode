mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_characters(a: &str, b: &str) -> i32 {
    let (n1, n2) = (a.len(), b.len());
    let mut res = n1 + n2;
    let [mut count1, mut count2] = [a, b].map(|s| {
        s.bytes().fold([0; 26], |mut acc, b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        })
    });
    for idx in 0..26 {
        // change all letters to idx
        res = res.min(n1 + n2 - count1[idx] - count2[idx]);
        if idx > 0 {
            // Prefix/accumulate all letters smaller than idx
            count1[idx] += count1[idx - 1];
            count2[idx] += count2[idx - 1];
        }
        if idx < 25 {
            // Change all letters in a to idx
            // And letters in b to (1+idx) => idx<25
            res = res.min(n1 - count1[idx] + count2[idx]);
            // Now change all in b to idx
            res = res.min(n2 - count2[idx] + count1[idx]);
        }
    }
    res as _
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(min_characters("aba", "caa"), 2);
        assert_eq!(min_characters("dabadd", "cda"), 3);
    }

    #[test]
    fn test() {}
}

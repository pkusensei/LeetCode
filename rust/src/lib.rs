mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_swaps(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let ones = s.iter().filter(|&&b| b == b'1').count();
    let zeros = n - ones;
    if ones.abs_diff(zeros) > 1 {
        return -1;
    }
    let mut temp = vec![];
    if n & 1 == 0 {
        while temp.len() < n {
            temp.extend_from_slice(b"01");
        }
        let count1 = temp.iter().zip(s).filter(|&(&a, &b)| a != b).count();
        temp.clear();
        while temp.len() < n {
            temp.extend_from_slice(b"10");
        }
        let count2 = temp.iter().zip(s).filter(|&(&a, &b)| a != b).count();
        count1.min(count2) as i32 / 2
    } else {
        while temp.len() < n {
            temp.extend_from_slice(if ones > zeros { b"10" } else { b"01" });
        }
        temp.pop();
        temp.iter().zip(s).filter(|&(&a, &b)| a != b).count() as i32 / 2
    }
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
        assert_eq!(min_swaps("111000"), 1);
        assert_eq!(min_swaps("010"), 0);
        assert_eq!(min_swaps("1110"), -1);
    }

    #[test]
    fn test() {
        assert_eq!(min_swaps("100"), 1);
    }
}

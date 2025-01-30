mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_binary_string(binary: String) -> String {
    let n = binary.len();
    let mut s = binary.into_bytes();
    let [mut ones, mut zeros] = [0, 0];
    // Given "111..11001010.."
    // Skip leading 1's and push zeros to the middle
    // Get "111...100..001111"
    for b in s.iter_mut() {
        if *b == b'0' {
            zeros += 1
        } else if zeros == 0 {
            ones += 1 // skip leading ones
        }
        *b = b'1'; // Try set all to 1's
    }
    // All 0's are pushed to the middle
    // All of them turned to 1's except last zero remains
    if ones < n {
        s[ones + zeros - 1] = b'0'
    }
    String::from_utf8(s).unwrap()
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
        assert_eq!(maximum_binary_string("000110".into()), "111011");
        assert_eq!(maximum_binary_string("01".into()), "01");
    }

    #[test]
    fn test() {}
}

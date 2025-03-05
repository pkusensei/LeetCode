mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_moves_to_make_palindrome(s: String) -> i32 {
    let n = s.len();
    let mut s = s.into_bytes();
    let [mut left, mut right] = [0, n - 1];
    let mut res = 0;
    while left < right {
        if s[left] != s[right] {
            let temp = right;
            while left < right && s[left] != s[right] {
                right -= 1;
            }
            if left == right {
                // s[left] is at the wrong spot
                s.swap(left, 1 + left);
                res += 1;
                right = temp;
            } else {
                s[right..=temp].rotate_left(1);
                res += temp - right;
                left += 1;
                right = temp - 1;
            }
        } else {
            left += 1;
            right -= 1;
        }
    }
    res as i32
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
        assert_eq!(min_moves_to_make_palindrome("aabb".into()), 2);
        assert_eq!(min_moves_to_make_palindrome("letelt".into()), 2);
    }

    #[test]
    fn test() {
        assert_eq!(
            min_moves_to_make_palindrome("skwhhaaunskegmdtutlgtteunmuuludii".into()),
            163
        );
    }
}

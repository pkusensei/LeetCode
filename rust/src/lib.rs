mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn decode_string(s: &str) -> String {
    let mut nums = vec![];
    let mut num = None;
    let mut stack = vec![];
    for &b in s.as_bytes() {
        match b {
            b'0'..=b'9' => {
                let n = 10 * num.unwrap_or(0) + i32::from(b - b'0');
                num = Some(n)
            }
            b'[' => {
                stack.push(b);
                nums.push(num.unwrap_or(1));
                num = None
            }
            b']' => {
                let mut temp = vec![];
                while stack.last().is_some_and(|&v| v != b'[') {
                    temp.push(stack.pop().unwrap());
                }
                stack.pop(); // '['
                let count = nums.pop().unwrap_or(1);
                temp.reverse();
                for _ in 0..count {
                    stack.extend_from_slice(&temp);
                }
            }
            _ => stack.push(b),
        }
    }
    stack.into_iter().map(char::from).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(decode_string("3[a]2[bc]"), "aaabcbc");
        debug_assert_eq!(decode_string("3[a2[c]]"), "accaccacc");
        debug_assert_eq!(decode_string("2[abc]3[cd]ef"), "abcabccdcdcdef");
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}

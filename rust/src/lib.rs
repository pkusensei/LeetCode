mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn compress(chars: &mut Vec<char>) -> i32 {
    let (mut left, mut right) = (0, 0);
    while right < chars.len() {
        if chars[right] == chars[left] {
            right += 1;
        } else if right - left > 1 {
            alter(&mut right, &mut left, chars);
        } else {
            left += 1;
        }
    }
    if (right - left) > 1 {
        alter(&mut right, &mut left, chars);
    }
    chars.len() as i32
}

fn alter(right: &mut usize, left: &mut usize, chars: &mut Vec<char>) {
    let s = (*right - *left).to_string();
    for digit in s.chars().rev() {
        chars.insert(*right, digit);
    }
    chars.drain(*left + 1..*right);
    *left += s.len() + 1;
    *right = *left;
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        {
            let mut chs = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
            debug_assert_eq!(compress(&mut chs), 6);
            debug_assert_eq!(chs, ['a', '2', 'b', '2', 'c', '3']);
        }
        {
            let mut chs = vec!['a'];
            debug_assert_eq!(compress(&mut chs), 1);
            debug_assert_eq!(chs, ['a']);
        }
        {
            let mut chs = vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
            ];
            debug_assert_eq!(compress(&mut chs), 4);
            debug_assert_eq!(chs, ['a', 'b', '1', '2']);
        }
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}

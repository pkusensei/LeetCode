mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn is_additive_number(num: &str) -> bool {
    let n = num.len();
    if n < 3 {
        return false;
    }

    let mut first = 0;
    let mut res = false;
    for i1 in 0..n - 2 {
        let mut second = 0;
        if i1 == 0 && num.as_bytes()[i1] == b'0' {
            for i2 in 1..n - 1 {
                if i2 == 1 && num.as_bytes()[i2] == b'0' {
                    return solve(num, i2 + 1, n, first, second);
                }
                second = num[1..=i2].parse().unwrap();
                res |= solve(num, i2 + 1, n, first, second);
                if res {
                    return true;
                }
            }
            break;
        }
        first = num[..=i1].parse::<i128>().unwrap();
        for i2 in i1 + 1..n - 1 {
            if i2 == i1 + 1 && num.as_bytes()[i2] == b'0' {
                res |= solve(num, i2 + 1, n, first, second);
                if !res {
                    break;
                }
            }
            second = num[i1 + 1..=i2].parse().unwrap();
            res |= solve(num, i2 + 1, n, first, second);
            if res {
                return true;
            }
        }
    }
    res
}

fn solve(num: &str, start: usize, n: usize, n1: i128, n2: i128) -> bool {
    if start == n {
        return true;
    }
    if num.as_bytes()[start] == b'0' {
        if n1 + n2 == 0 {
            return solve(num, start + 1, n, n2, 0);
        } else {
            return false;
        }
    }
    let mut res = false;
    for i in start..n {
        let v = num[start..=i].parse().unwrap();
        if v == n1 + n2 {
            res |= solve(num, i + 1, n, n2, v)
        }
        if res {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_additive_number("112358"));
        debug_assert!(is_additive_number("199100199"));
        debug_assert!(is_additive_number("000"));
        debug_assert!(!is_additive_number("0235813"));
        debug_assert!(!is_additive_number("1023"));
    }

    #[test]
    fn test() {
        debug_assert!(is_additive_number("198019823962"));
        debug_assert!(is_additive_number("11111111111011111111111"));
    }

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

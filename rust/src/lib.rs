mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn remove_kdigits(num: &str, k: i32) -> String {
    // with_stack(k, num)
    let mut count = k;
    let mut res = num.as_bytes().to_vec();
    let mut i = 0;
    while i < res.len() {
        if i == res.len() - 1 {
            break;
        }
        if res[i] > res[i + 1] {
            res.remove(i);
            count -= 1;
            continue;
        }
        if count == 0 {
            break;
        }
        i += 1
    }
    for _ in 0..count {
        res.pop();
    }
    while res.first().is_some_and(|&v| v == b'0') {
        res.remove(0);
    }
    if res.is_empty() {
        "0".to_string()
    } else {
        res.into_iter().map(char::from).collect()
    }
}

fn with_stack(k: i32, num: &str) -> String {
    let mut k = k as usize;
    let mut stack = Vec::with_capacity(num.len());
    for b in num.bytes() {
        while k > 0 && stack.last().is_some_and(|&v| v > b) {
            stack.pop();
            k -= 1
        }
        stack.push(b)
    }
    for _ in 0..k {
        stack.pop();
    }
    while stack.first().is_some_and(|&v| v == b'0') {
        stack.remove(0);
    }
    if stack.is_empty() {
        "0".to_string()
    } else {
        stack.into_iter().map(char::from).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(remove_kdigits("1432219", 3), "1219");
        debug_assert_eq!(remove_kdigits("10200", 1), "200");
        debug_assert_eq!(remove_kdigits("10", 2), "0");
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

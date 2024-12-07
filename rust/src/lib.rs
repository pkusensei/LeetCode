mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reverse_parentheses(s: &str) -> String {
    let mut stack = vec![];
    let mut curr = vec![];
    for b in s.bytes() {
        match b {
            b'(' => {
                stack.push(curr);
                curr = vec![];
            }
            b')' => {
                let mut prev = stack.pop().unwrap_or_default();
                prev.extend(curr.into_iter().rev());
                curr = prev;
            }
            _ => curr.push(b),
        }
    }
    String::from_utf8(curr).unwrap()
}

fn wormhole_teleport(s: &str) -> String {
    let n = s.len();
    let mut open = vec![];
    let mut pair = vec![0; n];
    for (i, b) in s.bytes().enumerate() {
        if b == b'(' {
            open.push(i);
        } else if b == b')' {
            let prev = open.pop().unwrap_or(0);
            pair[i] = prev;
            pair[prev] = i;
        }
    }
    let mut res = Vec::with_capacity(n);
    let mut idx = 0;
    let mut dir = true;
    while let Some(&b) = s.as_bytes().get(idx) {
        match b {
            b'(' | b')' => {
                idx = pair[idx];
                dir = !dir;
            }
            _ => res.push(b),
        }
        if dir {
            idx += 1;
        } else {
            idx -= 1;
        }
    }
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(wormhole_teleport("(abcd)"), "dcba");
        assert_eq!(wormhole_teleport("(u(love)i)"), "iloveu");
        assert_eq!(wormhole_teleport("(ed(et(oc))el)"), "leetcode");
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}

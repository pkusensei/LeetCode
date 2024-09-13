mod helper;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub fn deserialize(s: &str) -> NestedInteger {
    parse(s.as_bytes(), 0).0
}

fn parse(s: &[u8], mut idx: usize) -> (NestedInteger, usize) {
    let mut num = None;
    let mut list = vec![];
    let mut sign = 1;
    while idx < s.len() {
        match s[idx] {
            b'-' => sign = -1,
            b @ b'0'..=b'9' => {
                let n = 10 * num.unwrap_or(0) + (b - b'0') as i32;
                num = Some(n);
            }
            b',' => {
                if let Some(n) = num {
                    let v = NestedInteger::Int(n * sign);
                    list.push(v);
                    num = None;
                    sign = 1;
                }
            }
            b'[' => {
                let (v, i) = parse(s, idx + 1);
                list.push(v);
                idx = i;
            }
            b']' => {
                if let Some(n) = num {
                    let v = NestedInteger::Int(n * sign);
                    list.push(v);
                }
                return (NestedInteger::List(list), idx);
            }
            _ => todo!(),
        }
        idx += 1
    }
    if let Some(n) = num {
        (NestedInteger::Int(n * sign), idx)
    } else if let Some(v) = list.pop() {
        (v, idx) // top level list
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        use NestedInteger::*;
        // debug_assert_eq!(deserialize("324"), Int(324));
        debug_assert_eq!(
            deserialize("[123,[456,[789]]]"),
            List(vec![Int(123), List(vec![Int(456), List(vec![Int(789)])])])
        )
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

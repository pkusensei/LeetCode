mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn is_valid_serialization(preorder: &str) -> bool {
    // For a full tree
    // Number of leaves == 1+number of non-leaves
    let mut count = 1;
    for s in preorder.split(',') {
        if count <= 0 {
            return false;
        }
        if s == "#" {
            count -= 1
        } else {
            count += 1
        }
    }
    count == 0
}

fn with_stack(preorder: &str) -> bool {
    let mut iter = preorder.split(',').map(|s| s != "#");
    if iter.next().is_some_and(|v| !v) {
        // '#' as root can only stand alone
        return iter.next().is_none();
    };
    let mut stack = vec![false];
    while let Some(b) = iter.next() {
        if b {
            stack.push(false)
        } else {
            while let Some(last) = stack.last_mut() {
                // Every non-leaf node needs two leaves
                // one to mark its left branch is depleted
                // the other to pop
                if !*last {
                    *last = true;
                    break;
                }
                stack.pop();
            }
            if stack.is_empty() {
                return iter.next().is_none();
            }
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
        debug_assert!(is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#"));
        debug_assert!(!is_valid_serialization("1,#"));
        debug_assert!(!is_valid_serialization("9,#,#,1"));
    }

    #[test]
    fn test() {
        debug_assert!(!is_valid_serialization("#,#,3,5,#"));
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

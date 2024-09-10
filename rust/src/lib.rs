mod helper;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

fn expand(n: NestedInteger) -> Vec<i32> {
    let mut res = vec![];
    match n {
        NestedInteger::Int(n) => res.push(n),
        NestedInteger::List(n) => {
            for v in n.into_iter().map(expand) {
                res.extend(v)
            }
        }
    }
    res
}

struct NestedIterator {
    nums: Vec<i32>,
    idx: usize,
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let nums = nested_list.into_iter().map(expand).flatten().collect();
        Self { nums, idx: 0 }
    }

    fn next(&mut self) -> i32 {
        let res = self.nums[self.idx];
        self.idx += 1;
        res
    }

    fn has_next(&self) -> bool {
        self.idx < self.nums.len()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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

mod dsu;
mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_cost(nums: &[i32], k: i32, dist: i32) -> i64 {
    let [k, dist] = [k, dist].map(|v| v as usize);
    let [mut select, mut candids] = [const { MultiSet::new() }; 2];
    let mut curr = 0;
    for &num in nums[1..=1 + dist].iter() {
        curr += i64::from(num);
        select.insert(num);
    }
    curr = balance(&mut select, &mut candids, curr, k - 1);
    let mut res = curr;
    for (right, &num) in nums.iter().enumerate().skip(2 + dist) {
        let del = i64::from(nums[right - 1 - dist]);
        if select.remove(del) {
            curr -= del;
        } else {
            candids.remove(del);
        }
        if select
            .map
            .last_key_value()
            .is_some_and(|(&k, _)| k > i64::from(num))
        {
            curr += i64::from(num);
            select.insert(num);
        } else {
            candids.insert(num);
        }
        curr = balance(&mut select, &mut candids, curr, k - 1);
        res = res.min(curr);
    }
    res + i64::from(nums[0])
}

fn balance(select: &mut MultiSet, candids: &mut MultiSet, mut sum: i64, len: usize) -> i64 {
    while select.len < len {
        let Some(min) = candids.map.first_key_value().map(|(k, _)| *k) else {
            return sum;
        };
        select.insert(min);
        candids.remove(min);
        sum += min;
    }
    while select.len > len {
        let Some(max) = select.map.last_key_value().map(|(k, _)| *k) else {
            unreachable!()
        };
        select.remove(max);
        candids.insert(max);
        sum -= max;
    }
    sum
}

struct MultiSet {
    map: BTreeMap<i64, i32>,
    len: usize,
}

impl MultiSet {
    const fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            len: 0,
        }
    }

    fn insert(&mut self, num: impl Into<i64>) {
        *self.map.entry(num.into()).or_insert(0) += 1;
        self.len += 1;
    }

    fn remove(&mut self, num: impl Into<i64> + Copy) -> bool {
        if let Some(v) = self.map.get_mut(&num.into()) {
            *v -= 1;
            if *v == 0 {
                self.map.remove(&num.into());
            }
            self.len -= 1;
            return true;
        }
        false
    }
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
        assert_eq!(minimum_cost(&[1, 3, 2, 6, 4, 2], 3, 3), 5);
        assert_eq!(minimum_cost(&[10, 1, 2, 2, 2, 1], 4, 3), 15);
        assert_eq!(minimum_cost(&[10, 8, 18, 9], 3, 1), 36);
    }

    #[test]
    fn test() {}
}

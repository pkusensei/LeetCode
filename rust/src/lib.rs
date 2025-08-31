mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_product(nums: &[i32]) -> i64 {
    let mut res = 0;
    let mut t = Trie::default();
    for &num in nums.iter() {
        if let Some(v) = t.find(num, 20) {
            res = res.max(i64::from(v) * i64::from(num));
        }
        t.insert(num);
    }
    res
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 2],
    num: Option<i32>,
}

impl Trie {
    fn insert(&mut self, num: i32) {
        let mut curr = self;
        // rev works!
        for bit in (0..=20).rev() {
            let idx = ((num >> bit) & 1) as usize;
            curr = curr.nodes[idx].get_or_insert_default();
        }
        curr.num = Some(num);
    }

    fn find(&self, num: i32, bit: i32) -> Option<i32> {
        if bit < 0 {
            return self.num;
        }
        let idx = (num >> bit) & 1;
        if idx == 0 {
            let a = self.nodes[0].as_ref().and_then(|v| v.find(num, bit - 1));
            let b = self.nodes[1].as_ref().and_then(|v| v.find(num, bit - 1));
            match [a, b] {
                [Some(x), Some(y)] => Some(x.max(y)),
                _ => a.or(b),
            }
        } else {
            self.nodes[1 - idx as usize].as_ref()?.find(num, bit - 1)
        }
    }
}

pub fn with_dp(nums: &[i32]) -> i64 {
    let Some(&max) = nums.iter().max() else {
        return 0;
    };
    let width = 1 + max.ilog2();
    let mask = 1 << width;
    let mut dp = vec![0; mask];
    for &num in nums.iter() {
        dp[num as usize] = num;
    }
    for bit in 0..width {
        for m in 0..mask {
            // if `m` is set on `bit`, flip off a single bit to find
            // all max candidates with "stricter" bit patterns
            if (m & (1 << bit)) > 0 {
                dp[m] = dp[m].max(dp[m ^ (1 << bit)])
            }
        }
    }
    nums.iter()
        .map(|&num| {
            let complement = (mask - 1) as i32 ^ num;
            i64::from(num) * i64::from(dp[complement as usize])
        })
        .max()
        .unwrap_or(0)
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(max_product(&[1, 2, 3, 4, 5, 6, 7]), 12);
        assert_eq!(max_product(&[5, 6, 4]), 0);
        assert_eq!(max_product(&[64, 8, 32]), 2048);

        assert_eq!(with_dp(&[5, 6, 4]), 0);
        assert_eq!(with_dp(&[1, 2, 3, 4, 5, 6, 7]), 12);
        assert_eq!(with_dp(&[64, 8, 32]), 2048);
    }

    #[test]
    fn test() {}
}

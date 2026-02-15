mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_xor(nums: &[i32], k: i32) -> i32 {
    use std::collections::VecDeque;
    let mut minq = VecDeque::new();
    let mut maxq = VecDeque::new();
    let mut pref_xor = vec![0];
    let mut trie = Trie::default();
    trie.update(0, 1);
    let mut res = 0;
    let mut curr_xor = 0;
    let mut left = 0;
    for (right, &num) in nums.iter().enumerate() {
        curr_xor ^= num;
        pref_xor.push(curr_xor);
        trie.update(curr_xor, 1);
        while let Some((_, v)) = minq.back()
            && *v > num
        {
            minq.pop_back();
        }
        minq.push_back((right, num));
        while let Some((_, v)) = maxq.back()
            && *v < num
        {
            maxq.pop_back();
        }
        maxq.push_back((right, num));
        while let Some(&(left1, minv)) = minq.front()
            && let Some(&(left2, maxv)) = maxq.front()
            && maxv - minv > k
        {
            while left <= left1.min(left2) {
                trie.update(pref_xor[left], -1);
                left += 1;
            }
            while minq.front().is_some_and(|&(i, _)| i < left) {
                minq.pop_front();
            }
            while maxq.front().is_some_and(|&(i, _)| i < left) {
                maxq.pop_front();
            }
        }
        res = res.max(trie.query(curr_xor));
    }
    res
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 2],
    count: i32,
}

impl Trie {
    fn update(&mut self, num: i32, val: i32) {
        let mut curr = self;
        for bit in (0..16).rev() {
            let i = ((num >> bit) & 1) as usize;
            curr = curr.nodes[i].get_or_insert_default();
            curr.count += val;
        }
    }

    fn query(&self, num: i32) -> i32 {
        let mut curr = self;
        let mut res = 0;
        for bit in (0..16).rev() {
            let i = ((num >> bit) & 1) as usize;
            if let Some(node) = curr.nodes[1 - i].as_ref()
                && node.count > 0
            {
                res |= 1 << bit;
                curr = node;
            } else if let Some(node) = curr.nodes[i].as_ref() {
                curr = node;
            } else {
                break;
            }
        }
        res
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
        assert_eq!(max_xor(&[5, 4, 5, 6], 1), 6);
        assert_eq!(max_xor(&[5, 4, 5, 6], 2), 7);
    }

    #[test]
    fn test() {
        assert_eq!(max_xor(&[0, 3], 0), 3);
    }
}

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximize_xor(nums: &mut [i32], queries: &[[i32; 2]]) -> Vec<i32> {
    nums.sort_unstable();
    let n = queries.len();
    let mut query_indices: Vec<_> = (0..n).collect();
    query_indices.sort_unstable_by_key(|&i| queries[i][1]);
    let mut res = vec![-1; n];
    let mut numi = 0;
    let mut trie = Trie::new();
    for qi in query_indices {
        let [x, m] = [0, 1].map(|i| queries[qi][i]);
        while nums.get(numi).is_some_and(|&v| v <= m) {
            trie.insert(nums[numi]);
            numi += 1;
        }
        res[qi] = trie.find(x);
    }
    res
}

struct Trie {
    nodes: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: [None, None],
        }
    }

    fn insert(&mut self, num: i32) {
        let mut curr = self;
        for bit in (0..32).rev() {
            let idx = ((num >> bit) & 1) as usize;
            curr = curr.nodes[idx].get_or_insert(Box::new(Trie::new()));
        }
    }

    fn find(&self, x: i32) -> i32 {
        let mut curr = self;
        let mut res = 0;
        for bit in (0..32).rev() {
            let idx = ((x >> bit) & 1) as usize;
            if let Some(ref v) = curr.nodes[1 - idx] {
                // There is number at current bit different from x => xor this bit is 1
                res |= 1 << bit;
                curr = v
            } else if let Some(ref v) = curr.nodes[idx] {
                curr = v
            } else {
                return -1;
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
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(
            maximize_xor(&mut [0, 1, 2, 3, 4], &[[3, 1], [1, 3], [5, 6]]),
            [3, 3, 7]
        );
        assert_eq!(
            maximize_xor(&mut [5, 2, 4, 6, 6, 3], &[[12, 4], [8, 1], [6, 3]]),
            [15, -1, 5]
        );
    }

    #[test]
    fn test() {}
}

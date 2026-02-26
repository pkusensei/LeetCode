mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct RLEIterator {
    inner: Vec<[i32; 2]>, // [count, val]
}

impl RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        let v = encoding
            .chunks_exact(2)
            .rev()
            .filter_map(|w| if w[0] > 0 { Some([w[0], w[1]]) } else { None })
            .collect();
        Self { inner: v }
    }

    fn next(&mut self, mut n: i32) -> i32 {
        while let Some(&[count, _]) = self.inner.last()
            && count < n
        {
            self.inner.pop();
            n -= count;
        }
        if let Some(v) = self.inner.last_mut()
            && v[0] >= n
        {
            v[0] -= n;
            v[1]
        } else {
            -1
        }
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
    fn basics() {}

    #[test]
    fn test() {}
}

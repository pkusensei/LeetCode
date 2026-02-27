mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Default)]
struct StockSpanner {
    id: i32,
    st: Vec<(i32, i32)>, // (val, id)
}

impl StockSpanner {
    fn new() -> Self {
        Default::default()
    }

    fn next(&mut self, price: i32) -> i32 {
        while let Some(top) = self.st.last()
            && top.0 <= price
        {
            self.st.pop();
        }
        let res = self.id - self.st.last().map(|top| top.1).unwrap_or(-1);
        self.st.push((price, self.id));
        self.id += 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}

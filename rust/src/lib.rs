mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct Bank {
    data: Vec<i64>,
}

impl Bank {
    fn new(data: Vec<i64>) -> Self {
        Self { data }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let n = self.data.len();
        let a = account1 as usize;
        let b = account2 as usize;
        if (1..=n).contains(&a) && (1..=n).contains(&b) && self.data[a - 1] >= money {
            self.data[a - 1] -= money;
            self.data[b - 1] += money;
            true
        } else {
            false
        }
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let n = self.data.len();
        let a = account as usize;
        if (1..=n).contains(&a) {
            self.data[a - 1] += money;
            true
        } else {
            false
        }
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let n = self.data.len();
        let a = account as usize;
        if (1..=n).contains(&a) && self.data[a - 1] >= money {
            self.data[a - 1] -= money;
            true
        } else {
            false
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

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct Bank {
    data: Vec<i64>,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { data: balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let n = self.data.len() as i32;
        if (1..=n).contains(&account1) && (1..=n).contains(&account2) {
            let a1 = account1 as usize - 1;
            let a2 = account2 as usize - 1;
            if self.data[a1] >= money {
                self.data[a1] -= money;
                self.data[a2] += money;
                return true;
            }
        }
        false
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let n = self.data.len() as i32;
        if (1..=n).contains(&account) {
            let a = account as usize - 1;
            self.data[a] += money;
            return true;
        }
        false
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let n = self.data.len() as i32;
        if (1..=n).contains(&account) {
            let a = account as usize - 1;
            if self.data[a] >= money {
                self.data[a] -= money;
                return true;
            }
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
    fn basics() {}

    #[test]
    fn test() {}
}

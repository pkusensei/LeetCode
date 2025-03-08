mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct ATM {
    data: [i64; 5],
}

impl ATM {
    fn new() -> Self {
        Self { data: [0; 5] }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for (i, c) in banknotes_count.into_iter().enumerate() {
            self.data[i] += i64::from(c);
        }
    }

    fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        const DENS: [i64; 5] = [20, 50, 100, 200, 500];
        let mut amount = i64::from(amount);
        let mut temp = self.data;
        let mut res = vec![0; 5];
        for (i, (c, d)) in temp.iter_mut().zip(DENS).enumerate().rev() {
            if amount >= d && *c > 0 {
                let curr = (amount / d).min(*c);
                res[i] = curr as i32;
                *c -= curr;
                amount -= curr * d;
            }
        }
        if amount == 0 {
            self.data = temp;
            res
        } else {
            vec![-1]
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

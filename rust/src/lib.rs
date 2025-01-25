mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct Fancy {
    data: Vec<i64>,
    inc: i64,
    mul: i64,
}

const MOD: i64 = 1_000_000_007;

impl Fancy {
    fn new() -> Self {
        Self {
            data: vec![],
            inc: 0,
            mul: 1,
        }
    }

    fn append(&mut self, val: i32) {
        let num = (i64::from(val) - self.inc).rem_euclid(MOD) * mod_pow(self.mul, MOD - 2) % MOD;
        self.data.push(num);
    }

    fn add_all(&mut self, inc: i32) {
        self.inc = (self.inc + i64::from(inc)) % MOD
    }

    fn mult_all(&mut self, m: i32) {
        let m = i64::from(m);
        self.mul = (self.mul * m) % MOD;
        self.inc = (self.inc * m) % MOD;
    }

    fn get_index(&self, idx: i32) -> i32 {
        let Some(&num) = self.data.get(idx as usize) else {
            return -1;
        };
        let res = (num * self.mul + self.inc) % MOD;
        res as _
    }
}

const fn mod_pow(x: i64, y: i64) -> i64 {
    let mut res = 1;
    let mut p = x;
    let mut y = y;
    while y > 0 {
        if y & 1 == 1 {
            res = res * p % MOD;
        }
        p = p.pow(2) % MOD;
        y >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

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

    #[test]
    fn basics() {
        let mut fancy = Fancy::new();
        fancy.append(2); // fancy sequence: [2]
        fancy.add_all(3); // fancy sequence: [2+3] -> [5]
        fancy.append(7); // fancy sequence: [5, 7]
        fancy.mult_all(2); // fancy sequence: [5*2, 7*2] -> [10, 14]
        assert_eq!(fancy.get_index(0), 10); // return 10
        fancy.add_all(3); // fancy sequence: [10+3, 14+3] -> [13, 17]
        fancy.append(10); // fancy sequence: [13, 17, 10]
        fancy.mult_all(2); // fancy sequence: [13*2, 17*2, 10*2] -> [26, 34, 20]
        assert_eq!(fancy.get_index(0), 26); // return 26
        assert_eq!(fancy.get_index(1), 34); // return 34
        assert_eq!(fancy.get_index(2), 20); // return 20
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}

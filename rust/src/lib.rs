mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct Fancy {
    data: Vec<i64>,
    add: i64,
    mul: i64,
}

impl Fancy {
    fn new() -> Self {
        Self {
            data: vec![],
            add: 0,
            mul: 1,
        }
    }

    fn append(&mut self, val: i32) {
        let mut num = (i64::from(val) - self.add).rem_euclid(M);
        num = (num * mod_pow(self.mul, M - 2)) % M;
        self.data.push(num);
    }

    fn add_all(&mut self, inc: i32) {
        self.add = (self.add + i64::from(inc)) % M;
    }

    fn mult_all(&mut self, m: i32) {
        let m = i64::from(m);
        self.add = (self.add * m) % M;
        self.mul = (self.mul * m) % M;
    }

    fn get_index(&self, idx: i32) -> i32 {
        let Some(&num) = self.data.get(idx as usize) else {
            return -1;
        };
        ((num * self.mul + self.add) % M) as i32
    }
}

const M: i64 = 1_000_000_007;

const fn mod_pow(b: i64, e: i64) -> i64 {
    if e == 0 {
        return 1;
    }
    if e & 1 == 0 {
        mod_pow(b * b % M, e >> 1)
    } else {
        mod_pow(b * b % M, e >> 1) * b % M
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

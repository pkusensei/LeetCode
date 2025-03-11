mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct BookMyShow {
    m: i32,
    rows: Vec<i32>,
    start: usize,
}

impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        Self {
            m,
            rows: vec![m; n as usize],
            start: 0,
        }
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        for i in self.start..=max_row as usize {
            if self.rows[i] >= k {
                self.rows[i] -= k;
                return vec![i as i32, self.m - self.rows[i] - k];
            }
        }
        vec![]
    }

    fn scatter(&mut self, mut k: i32, max_row: i32) -> bool {
        let mut idx = self.start;
        while idx <= max_row as usize {
            if self.rows[idx] > k {
                break;
            }
            k -= self.rows[idx];
            idx += 1;
        }
        if idx >= 1 + max_row as usize {
            if k == 0 {
                self.start = idx;
                return true;
            }
            return false;
        }
        self.start = idx;
        self.rows[idx] -= k;
        true
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
    fn basics() {
        let mut bms = BookMyShow::new(2, 5); // There are 2 rows with 5 seats each
        assert_eq!(bms.gather(4, 0), [0, 0]); // return [0, 0]
                                              // The group books seats [0, 3] of row 0.
        assert!(bms.gather(2, 0).is_empty()); // return []
                                              // There is only 1 seat left in row 0,
                                              // so it is not possible to book 2 consecutive seats.
        assert!(bms.scatter(5, 1)); // return True
                                    // The group books seat 4 of row 0 and seats [0, 3] of row 1.
        assert!(!bms.scatter(5, 1));
    }

    #[test]
    fn test() {}
}

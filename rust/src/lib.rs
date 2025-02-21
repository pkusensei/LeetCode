mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

struct DetectSquares {
    pts: HashMap<[i32; 2], i32>,
}

impl DetectSquares {
    fn new() -> Self {
        Self {
            pts: HashMap::new(),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        *self.pts.entry([point[0], point[1]]).or_insert(0) += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let [x0, y0] = point[..] else {
            return 0;
        };
        let mut res = 0;
        for &[x1, y1] in self.pts.keys().filter(|&&[x1, y1]| x1 == x0 && y1 != y0) {
            for &[x2, y2] in self.pts.keys().filter(|&&[x2, y2]| x2 != x0 && y2 == y0) {
                if y1.abs_diff(y0) == x2.abs_diff(x0) {
                    res += self.pts[&[x1, y1]]
                        * self.pts[&[x2, y2]]
                        * self.pts.get(&[x2, y1]).unwrap_or(&0);
                }
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

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct Robot {
    idx: usize,
    robs: Vec<([i32; 2], &'static str)>,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        let mut robs = vec![([0, 0], "South")];
        robs.extend((1..width).map(|x| ([x, 0], "East")));
        robs.extend((1..height).map(|y| ([width - 1, y], "North")));
        robs.extend((0..width - 1).rev().map(|x| ([x, height - 1], "West")));
        robs.extend((1..height - 1).rev().map(|y| ([0, y], "South")));
        Self { idx: 0, robs }
    }

    fn step(&mut self, num: i32) {
        self.idx += num as usize
    }

    fn get_pos(&self) -> Vec<i32> {
        self.robs[self.idx % self.robs.len()].0.to_vec()
    }

    fn get_dir(&self) -> String {
        if self.idx == 0 {
            "East".to_string()
        } else {
            self.robs[self.idx % self.robs.len()].1.to_string()
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

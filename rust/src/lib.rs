mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct Robot {
    pos: Vec<(i32, i32, u8)>,
    idx: usize,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        let mut pos = vec![(0, 0, 3)];
        pos.extend((1..width).map(|x| (x, 0, 0)));
        pos.extend((1..height).map(|y| (width - 1, y, 1)));
        pos.extend((0..width - 1).map(|x| (x, height - 1, 2)).rev());
        pos.extend((1..height - 1).map(|y| (0, y, 3)).rev());
        Self { pos, idx: 0 }
    }

    fn step(&mut self, num: i32) {
        self.idx += num as usize
    }

    fn get_pos(&self) -> Vec<i32> {
        let i = self.idx % self.pos.len();
        let v = self.pos[i];
        vec![v.0, v.1]
    }

    fn get_dir(&self) -> String {
        if self.idx == 0 {
            return "East".to_string();
        }
        let i = self.idx % self.pos.len();
        let dir = self.pos[i].2;
        match dir {
            0 => "East",
            1 => "North",
            2 => "West",
            _ => "South",
        }
        .to_string()
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

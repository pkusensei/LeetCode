mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut masks = Masks::default();
    for (r, row) in board.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if let Some(v) = ch.to_digit(10) {
                masks.add(r, c, v);
            }
        }
    }
    backtrack(board, &mut masks);
}

const N: usize = 9;

fn backtrack(board: &mut Vec<Vec<char>>, masks: &mut Masks) -> bool {
    for r in 0..N {
        for c in 0..N {
            if board[r][c] == '.' {
                for num in 1..=9 {
                    if masks.check(r, c, num) {
                        board[r][c] = char::from_digit(num, 10).unwrap();
                        masks.add(r, c, num);
                        if backtrack(board, masks) {
                            return true;
                        }
                        board[r][c] = '.';
                        masks.remove(r, c, num);
                    }
                }
                return false;
            }
        }
    }
    true
}

#[derive(Default)]
struct Masks {
    row_masks: [u16; N],
    col_masks: [u16; N],
    box_masks: [[u16; 3]; 3],
}

impl Masks {
    fn add(&mut self, r: usize, c: usize, v: u32) {
        self.row_masks[r] |= 1 << v;
        self.col_masks[c] |= 1 << v;
        self.box_masks[r / 3][c / 3] |= 1 << v;
    }

    fn check(&self, r: usize, c: usize, v: u32) -> bool {
        (self.row_masks[r] >> v) & 1 == 0
            && (self.col_masks[c] >> v) & 1 == 0
            && (self.box_masks[r / 3][c / 3] >> v) & 1 == 0
    }

    fn remove(&mut self, r: usize, c: usize, v: u32) {
        self.row_masks[r] ^= 1 << v;
        self.col_masks[c] ^= 1 << v;
        self.box_masks[r / 3][c / 3] ^= 1 << v;
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

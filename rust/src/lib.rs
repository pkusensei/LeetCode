mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool {
    let [rows, cols] = get_dimensions(&board);
    let mut transposed = vec![vec![' '; rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            transposed[c][r] = board[r][c];
        }
    }
    if find(&board, word.as_bytes()) || find(&transposed, word.as_bytes()) {
        return true;
    }
    let mut s = word.into_bytes();
    s.reverse();
    find(&board, &s) || find(&transposed, &s)
}

fn find(board: &[Vec<char>], s: &[u8]) -> bool {
    let [rows, cols] = get_dimensions(board);
    let mut count = vec![vec![-1; cols]; rows]; // horizontal match
    for r in 0..rows {
        for c in 0..cols {
            if c == 0 || board[r][c - 1] == '#' || (c > 0 && count[r][c - 1] > -1) {
                let pos = if c == 0 {
                    0
                } else {
                    1 + count[r][c - 1] as usize
                };
                if board[r][c] == ' ' || board[r][c] == char::from(s[pos]) {
                    if pos == s.len() - 1 {
                        if c == cols - 1 || board[r][1 + c] == '#' {
                            return true;
                        }
                    } else {
                        count[r][c] = pos as i32;
                    }
                }
            }
        }
    }
    false
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

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
    for [dr, dc] in ALL_DIRS {
        for step in 1.. {
            let nr = r_move + step * dr;
            let nc = c_move + step * dc;
            if (0..8).contains(&nr) && (0..8).contains(&nc) {
                if board[nr as usize][nc as usize] == '.' {
                    break;
                }
                if board[nr as usize][nc as usize] == color {
                    if step > 1 {
                        return true;
                    }
                    break;
                }
            } else {
                break;
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

mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn find_min_step(board: &str, hand: &str) -> i32 {
    let board: Vec<_> = board.bytes().collect();
    let mut hand: Vec<_> = hand.bytes().collect();
    hand.sort_unstable();
    let mut queue = VecDeque::from([(board, hand, 0)]);
    let mut seen = HashSet::new();
    while let Some((board, hand, dist)) = queue.pop_front() {
        if board.is_empty() {
            return dist;
        }
        if !seen.insert((board.clone(), hand.clone())) {
            continue;
        }
        for i1 in 0..board.len() {
            for i2 in 0..hand.len() {
                if i2 > 0 && hand[i2 - 1] == hand[i2] {
                    continue;
                }
                if i1 > 0 && board[i1 - 1] == hand[i2] {
                    continue;
                }
                let pick = board[i1] == hand[i2] || (i1 > 0 && board[i1 - 1] == board[i1]); // && board[i1] != hand[i2]
                if pick {
                    let mut new_board = board.clone();
                    new_board.insert(i1, hand[i2]);
                    let mut new_hand = hand.clone();
                    new_hand.remove(i2);
                    new_board = process(new_board, i1);
                    queue.push_back((new_board, new_hand, dist + 1));
                }
            }
        }
    }
    -1
}

fn process(mut board: Vec<u8>, idx: usize) -> Vec<u8> {
    let n = board.len();
    if idx >= n {
        return board;
    }
    let (mut left, mut right) = (idx, idx);
    while left > 0 && board[left - 1] == board[idx] {
        left -= 1;
    }
    while right + 1 < n && board[right + 1] == board[idx] {
        right += 1;
    }
    if right - left + 1 >= 3 {
        board.drain(left..=right);
        process(board, left)
    } else {
        board
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_min_step("WRRBBW", "RB"), -1);
        debug_assert_eq!(find_min_step("WWRRBBWW", "WRBRW"), 2);
        debug_assert_eq!(find_min_step("G", "GGGGG"), 2);
    }

    #[test]
    fn test() {
        debug_assert_eq!(find_min_step("RRWWRRBBRR", "WB"), 2);
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}

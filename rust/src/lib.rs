mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn stone_game_ix(stones: &[i32]) -> bool {
    let [mut zeros, mut ones, mut twos] = [0i32; 3];
    for &num in stones.iter() {
        match num % 3 {
            0 => zeros += 1,
            1 => ones += 1,
            _ => twos += 1,
        }
    }
    // zeros only affects turns
    if zeros & 1 == 0 {
        // With a mix of 1s and 2s, Alice always starts with lesser count
        // So that Bob will be forced to make a 3
        // If either is empty, Alice making the third move loses
        ones.min(twos) != 0
    } else {
        // Alice always starts with bigger count
        // abs(ones-zeros)<=2 => never making a 3, Bob wins
        ones.abs_diff(twos) > 2
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
        assert!(stone_game_ix(&[2, 1]));
        assert!(!stone_game_ix(&[2]));
        assert!(!stone_game_ix(&[5, 1, 2, 4, 3]));
    }

    #[test]
    fn test() {
        assert!(!stone_game_ix(&[3, 3]));
    }
}

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
    const N: usize = 12;
    let mut max = 0;
    let mut res = vec![];
    'outer: for mask in 0..(1 << N) {
        let mut num = num_arrows;
        let mut curr = [0; N];
        let mut score = 0;
        for idx in 0..N {
            if (mask >> idx) & 1 == 0 {
                num -= alice_arrows[idx] + 1;
                if num < 0 {
                    continue 'outer;
                }
                curr[idx] = alice_arrows[idx] + 1;
                score += idx as i32;
            }
        }
        if num >= 0 && score > max {
            max = score;
            curr[11] += num;
            res = curr.to_vec();
        }
    }
    res
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

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn decode(encoded: &[i32]) -> Vec<i32> {
    // encoded: [a^b, b^c, c^d, d^e...]
    // all odds: [b^c, d^e..]
    // xor ^ b^c ^ d^e
    let n = encoded.len() as i32 + 1;
    let xor = (1..=n).fold(0, |acc, num| acc ^ num);
    let mut curr = encoded
        .iter()
        .skip(1)
        .step_by(2)
        .fold(xor, |acc, &num| acc ^ num);
    let mut res = vec![curr];
    for num in encoded.iter() {
        curr ^= num;
        res.push(curr);
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(decode(&[3, 1]), [1, 2, 3]);
    }

    #[test]
    fn test() {}
}

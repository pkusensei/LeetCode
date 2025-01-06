mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(boxes: &str) -> Vec<i32> {
    let n = boxes.len();
    let mut prefix = Vec::with_capacity(n);
    for b in boxes.bytes() {
        prefix.push(i32::from(b - b'0') + prefix.last().unwrap_or(&0));
    }
    let ones = prefix[n - 1];
    let mut res = Vec::with_capacity(n);
    let mut curr: i32 = boxes
        .bytes()
        .enumerate()
        .skip(1)
        .filter_map(|(i, b)| if b == b'1' { Some(i as i32) } else { None })
        .sum();
    res.push(curr);
    for (i, b) in boxes.bytes().enumerate().skip(1) {
        curr += prefix[i - 1];
        curr -= ones - prefix[i];
        if b == b'1' {
            curr -= 1;
        }
        res.push(curr);
    }
    res
}

fn single_pass(boxes: &str) -> Vec<i32> {
    let n = boxes.len();
    let mut res = vec![0; n];
    let [mut balls_left, mut moves_to_left, mut balls_right, mut moves_to_right] = [0; 4];
    for (i1, b) in boxes.bytes().enumerate() {
        res[i1] += moves_to_left;
        balls_left += i32::from(b - b'0');
        moves_to_left += balls_left;

        let i2 = n - 1 - i1;
        res[i2] += moves_to_right;
        balls_right += i32::from(boxes.as_bytes()[i2] - b'0');
        moves_to_right += balls_right;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_operations("110"), [1, 1, 3]);
        assert_eq!(min_operations("001011"), [11, 8, 5, 4, 3, 4]);

        assert_eq!(single_pass("110"), [1, 1, 3]);
        assert_eq!(single_pass("001011"), [11, 8, 5, 4, 3, 4]);
    }

    #[test]
    fn test() {}

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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}

mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn complex_number_multiply(num1: &str, num2: &str) -> String {
    let [a1, b1] = parse(num1);
    let [a2, b2] = parse(num2);
    let real = a1 * a2 - b1 * b2;
    let imag = a1 * b2 + b1 * a2;
    format!("{real}+{imag}i")
}

fn parse(s: &str) -> [i16; 2] {
    let (a, b) = s.split_once("+").unwrap();
    [a.parse().unwrap(), b.trim_end_matches('i').parse().unwrap()]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(complex_number_multiply("1+1i", "1+1i"), "0+2i");
        debug_assert_eq!(complex_number_multiply("1+-1i", "1+-1i"), "0+-2i");
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
}

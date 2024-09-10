mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn max_envelopes(envelopes: &mut [[i32; 2]]) -> i32 {
    envelopes.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let mut values = vec![envelopes[0][1]];
    // longest increasing sequence
    for &[_, num] in envelopes.iter() {
        if num > *values.last().unwrap() {
            values.push(num)
        } else if let Err(i) = values.binary_search(&num) {
            values[i] = num;
        }
    }
    values.len() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_envelopes(&mut [[5, 4], [6, 4], [6, 7], [2, 3]]), 3);
        debug_assert_eq!(max_envelopes(&mut [[1, 1], [1, 1], [1, 1]]), 1);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}

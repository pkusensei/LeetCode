mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn deck_revealed_increasing(deck: &mut [i32]) -> Vec<i32> {
    deck.sort_unstable_by(|a, b| b.cmp(a));
    let mut queue = std::collections::VecDeque::with_capacity(deck.len());
    for &num in deck.iter() {
        if let Some(v) = queue.pop_back() {
            queue.push_front(v);
        }
        queue.push_front(num);
    }
    queue.into()
}

fn with_two_pointers(deck: &mut [i32]) -> Vec<i32> {
    let n = deck.len();
    let mut res = vec![0; n];
    let mut skip = false;
    let (mut in_deck, mut in_res) = (0, 0);
    deck.sort_unstable();
    while in_deck < n {
        if res[in_res] == 0 {
            if !skip {
                res[in_res] = deck[in_deck];
                in_deck += 1;
            }
            skip = !skip;
        }
        in_res = (in_res + 1) % n;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            with_two_pointers(&mut [17, 13, 11, 2, 3, 5, 7]),
            [2, 13, 3, 11, 5, 17, 7]
        );
        debug_assert_eq!(with_two_pointers(&mut [1, 1000]), [1, 1000]);
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}

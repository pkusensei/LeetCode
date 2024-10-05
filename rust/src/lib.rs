mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn shopping_offers(price: &[i32], special: &mut [&[i32]], needs: &[i32]) -> i32 {
    let n = price.len();
    // min cost with max goodies
    special.sort_unstable_by(|a, b| a[n].cmp(&b[n]).then(b[..n].cmp(&a[..n])));
    dfs(price, special, needs, &mut HashMap::new())
}

fn dfs(price: &[i32], special: &[&[i32]], needs: &[i32], seen: &mut HashMap<i32, i32>) -> i32 {
    let bits = to_bits(needs);
    if let Some(&v) = seen.get(&bits) {
        return v;
    }
    let n = price.len();
    match special {
        [] => {
            debug_assert_eq!(n, needs.len());
            price.iter().zip(needs.iter()).map(|(a, b)| a * b).sum()
        }
        [head, tail @ ..] => {
            let mut res = i32::MAX;
            // take as many offers as possible
            if let Some(curr) = pick(needs, head) {
                let take =
                    head[n] + dfs(price, special, &curr, seen).min(dfs(price, tail, &curr, seen));
                res = res.min(take);
            }
            res = res.min(dfs(price, tail, needs, seen)); // skip
            seen.insert(bits, res);
            res
        }
    }
}

fn pick(needs: &[i32], special: &[i32]) -> Option<Vec<i32>> {
    debug_assert_eq!(needs.len() + 1, special.len());
    needs
        .iter()
        .zip(special.iter())
        .map(|(a, b)| if a < b { None } else { Some(a - b) })
        .collect()
}

fn to_bits(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, &num| (acc << 4) | num)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            shopping_offers(&[2, 5], &mut [&[3, 0, 5], &[1, 2, 10]], &[3, 2],),
            14
        );
        debug_assert_eq!(
            shopping_offers(&[2, 3, 4], &mut [&[1, 1, 0, 4], &[2, 2, 1, 9]], &[1, 2, 1]),
            11
        );
    }

    #[test]
    fn test() {
        #[rustfmt::skip]
        debug_assert_eq!(
            shopping_offers(
                &[2, 2],
                &mut [
                    &[1, 1, 1],&[1, 1, 2],&[1, 1, 3],&[1, 1, 4],&[1, 1, 5],
                    &[1, 1, 6],&[1, 1, 7],&[1, 1, 8],&[1, 1, 9],&[1, 1, 10],
                    &[1, 1, 11],&[1, 1, 12],&[1, 1, 13],&[1, 1, 14],&[1, 1, 15]
                ],
                &[10, 10]
            ),
            10
        );
        debug_assert_eq!(
            shopping_offers(
                &[4, 9, 6],
                &mut [&[0, 1, 1, 5], &[2, 3, 2, 1], &[1, 2, 1, 7], &[1, 0, 0, 2]],
                &[4, 5, 3]
            ),
            10
        );
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

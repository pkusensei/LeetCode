mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn permute(n: i32, k: i64) -> Vec<i32> {
    let n = n as usize;
    let mut res = Vec::with_capacity(n);
    let type_ = if n & 1 == 1 { Some(1) } else { None };
    construct(
        n,
        k as _,
        type_,
        &mut vec![false; 1 + n],
        &mut vec![false; 1 + n],
        &mut res,
    );
    res
}

fn construct(
    n: usize,
    mut k: usize,
    type_: Option<usize>,
    seen_odd: &mut [bool],
    seen_even: &mut [bool],
    res: &mut Vec<i32>,
) {
    let count_odd = seen_odd
        .iter()
        .enumerate()
        .filter(|&(i, &v)| i & 1 == 1 && !v)
        .count();
    let count_even = seen_even
        .iter()
        .enumerate()
        .filter(|&(i, &v)| i > 0 && i & 1 == 0 && !v)
        .count();
    if count_odd + count_even == 0 {
        return;
    }
    let next_perm = if let Some(t) = type_ {
        count(count_odd - t, count_even - (1 - t))
    } else {
        count(count_odd, count_even - 1)
    };
    for num in 1..=n {
        if type_.is_none_or(|t| t == num & 1) {
            if (num & 1 == 1 && seen_odd[num]) || (num & 1 == 0 && seen_even[num]) {
                continue;
            }
            if next_perm.is_some_and(|v| v < k) {
                k -= next_perm.unwrap();
            } else {
                if num & 1 == 1 {
                    seen_odd[num] = true;
                } else {
                    seen_even[num] = true;
                }
                res.push(num as i32);
                construct(n, k, Some(1 - (num & 1)), seen_odd, seen_even, res);
                break;
            }
        }
    }
}

fn count(odd: usize, even: usize) -> Option<usize> {
    const F: [usize; 21] = {
        let mut f = [1; 21];
        let mut i = 1;
        while i < 21 {
            f[i] = i * f[i - 1];
            i += 1;
        }
        f
    };
    let fodd = F.get(odd);
    let feven = F.get(even);
    fodd.zip(feven).and_then(|(a, b)| a.checked_mul(*b))
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(permute(4, 6), [3, 4, 1, 2]);
        assert_eq!(permute(3, 2), [3, 2, 1]);
        assert_eq!(permute(2, 3), []);
    }

    #[test]
    fn test() {}
}

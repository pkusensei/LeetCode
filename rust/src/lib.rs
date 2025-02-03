mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_eat(candies_count: &[i32], queries: &[[i32; 3]]) -> Vec<bool> {
    let prefix = candies_count.iter().fold(vec![0], |mut acc, &num| {
        acc.push(i64::from(num) + acc.last().unwrap_or(&0));
        acc
    });
    let mut res = vec![];
    for q in queries.iter() {
        let favt = q[0];
        let favd = q[1];
        let cap = q[2];
        let early = prefix[favt as usize] / i64::from(cap);
        let late = prefix[favt as usize + 1] - 1;
        res.push((early..=late).contains(&i64::from(favd)));
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
        assert_eq!(
            can_eat(
                &[7, 4, 5, 3, 8],
                &[[0, 2, 2], [4, 2, 4], [2, 13, 1000000000]]
            ),
            [true, false, true]
        );
        assert_eq!(
            can_eat(
                &[5, 2, 6, 4, 1],
                &[[3, 1, 2], [4, 10, 3], [3, 10, 100], [4, 100, 30], [1, 3, 1]]
            ),
            [false, true, true, false, false]
        );
    }

    #[test]
    fn test() {}
}

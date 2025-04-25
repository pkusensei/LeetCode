mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut prefix = vec![];
    for row in grid.iter() {
        let mut curr = row.iter().fold(vec![], |mut acc, &v| {
            acc.push(i64::from(v) + acc.last().unwrap_or(&0));
            acc
        });
        if let Some(up) = prefix.last() {
            for (v, old) in curr.iter_mut().zip(up) {
                *v += old
            }
        }
        prefix.push(curr);
    }
    prefix
        .iter()
        .flatten()
        .filter(|&&v| v <= i64::from(k))
        .count() as i32
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

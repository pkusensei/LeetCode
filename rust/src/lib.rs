mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
    let cols = grid[0].len();
    let mut prev = vec![None::<i32>; cols];
    let mut res = 0;
    for row in grid.iter() {
        let mut curr = row.iter().fold(vec![], |mut acc, &ch| {
            let v = acc.last().unwrap_or(&None);
            match ch {
                'X' => acc.push(Some(v.unwrap_or(0) + 1)),
                'Y' => acc.push(Some(v.unwrap_or(0) - 1)),
                _ => acc.push(v.clone()),
            }
            acc
        });
        for (v, p) in curr.iter_mut().zip(&prev) {
            *v = match (*v, p) {
                (None, None) => None,
                _ => Some(v.unwrap_or(0) + p.unwrap_or(0)),
            };
            res += i32::from(v.is_some_and(|num| num == 0));
        }
        prev = curr;
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
    fn basics() {}

    #[test]
    fn test() {}
}

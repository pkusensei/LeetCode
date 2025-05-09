mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
    let mut prev: Vec<[i32; 2]> = vec![];
    let mut res = 0;
    for row in grid.iter() {
        let mut curr = vec![];
        for (c, &v) in row.iter().enumerate() {
            let mut temp = *curr.last().unwrap_or(&[0, 0]);
            if v == 'X' {
                temp[0] += 1;
            }
            if v == 'Y' {
                temp[1] += 1;
            }
            if let Some(up) = prev.get(c) {
                temp[0] += up[0];
                temp[1] += up[1];
                if c > 0 {
                    temp[0] -= prev[c - 1][0];
                    temp[1] -= prev[c - 1][1]
                }
            }
            res += i32::from(temp[0] > 0 && temp[0] == temp[1]);
            curr.push(temp);
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
    fn basics() {
        assert_eq!(
            number_of_submatrices(vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']]),
            3
        );
    }

    #[test]
    fn test() {}
}

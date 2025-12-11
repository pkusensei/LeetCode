mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut max_row = vec![0; 1 + n];
    let mut min_row = vec![1 + n; 1 + n];
    let mut max_col = vec![0; 1 + n];
    let mut min_col = vec![1 + n; 1 + n];
    for b in &buildings {
        let [x, y] = b[..] else { unreachable!() };
        max_row[y as usize] = max_row[y as usize].max(x as usize);
        min_row[y as usize] = min_row[y as usize].min(x as usize);
        max_col[x as usize] = max_col[x as usize].max(y as usize);
        min_col[x as usize] = min_col[x as usize].min(y as usize);
    }
    let mut res = 0;
    for b in buildings {
        let [x, y] = b[..] else { unreachable!() };
        if (1 + min_row[y as usize]..max_row[y as usize]).contains(&(x as usize))
            && (1 + min_col[x as usize]..max_col[x as usize]).contains(&(y as usize))
        {
            res += 1;
        }
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

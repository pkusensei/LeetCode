mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
    let mut res = i64::MIN;
    let mut map = HashMap::new();
    for (r1, row1) in board.iter().enumerate() {
        let vals1 = big3(row1, r1, &mut map);
        for (r2, row2) in board.iter().enumerate().skip(1 + r1) {
            let vals2 = big3(row2, r2, &mut map);
            for (r3, row3) in board.iter().enumerate().skip(1 + r2) {
                let vals3 = big3(row3, r3, &mut map);
                let curr = vals1
                    .into_iter()
                    .cartesian_product(vals2)
                    .cartesian_product(vals3)
                    .filter_map(|(((c1, v1), (c2, v2)), (c3, v3))| {
                        if c1 != c2 && c2 != c3 && c3 != c1 {
                            Some(i64::from(v1) + i64::from(v2) + i64::from(v3))
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap_or(i64::MIN);
                res = res.max(curr);
            }
        }
    }
    res
}

fn big3(row: &[i32], r: usize, map: &mut HashMap<usize, [(usize, i32); 3]>) -> [(usize, i32); 3] {
    if let Some(&v) = map.get(&r) {
        return v;
    }
    let [mut i1, mut i2, mut i3] = [0; 3];
    let [mut v1, mut v2, mut v3] = [i32::MIN; 3];
    for (i, &num) in row.iter().enumerate() {
        if num >= v1 {
            (i3, v3) = (i2, v2);
            (i2, v2) = (i1, v1);
            (i1, v1) = (i, num);
        } else if num >= v2 {
            (i3, v3) = (i2, v2);
            (i2, v2) = (i, num);
        } else if num > v3 {
            (i3, v3) = (i, num);
        }
    }
    let v = [(i1, v1), (i2, v2), (i3, v3)];
    map.insert(r, v);
    v
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

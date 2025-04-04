mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_increasing_cells(mat: &[&[i32]]) -> i32 {
    use std::collections::BTreeMap;
    let mut map = BTreeMap::<_, Vec<_>>::new();
    let [mut rows, mut cols] = [0, 0];
    for (r, row) in mat.iter().enumerate() {
        rows = rows.max(1 + r);
        for (c, &v) in row.iter().enumerate() {
            cols = cols.max(1 + c);
            map.entry(v).or_default().push([r, c]);
        }
    }
    // record max increments per row or col
    let mut row_inc = vec![0; rows];
    let mut col_inc = vec![0; cols];
    let mut res = 0;
    for pts in map.values() {
        // local increments per grid cell
        let mut curr_inc = vec![];
        for &[r, c] in pts {
            // potential increments if path crosses this point
            let val = 1 + row_inc[r].max(col_inc[c]);
            curr_inc.push(val);
            res = res.max(val);
        }
        for (&val, &[r, c]) in curr_inc.iter().zip(pts) {
            // update local increments into global arrays
            row_inc[r] = row_inc[r].max(val);
            col_inc[c] = col_inc[c].max(val);
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
        assert_eq!(max_increasing_cells(&[&[3, 1], &[3, 4]]), 2);
        assert_eq!(max_increasing_cells(&[&[1, 1], &[1, 1]]), 1);
        assert_eq!(max_increasing_cells(&[&[3, 1, 6], &[-9, 5, 7]]), 4);
    }

    #[test]
    fn test() {}
}

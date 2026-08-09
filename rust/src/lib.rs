mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_area(mat: &[&[i32]]) -> i32 {
    let [rows, cols] = get_dimensions(&mat);
    let mut prefix = vec![vec![0; 1 + cols]];
    for (r, row) in mat.iter().enumerate() {
        let mut curr = row.iter().fold(vec![0], |mut acc, v| {
            acc.push(v + acc.last().unwrap_or(&0));
            acc
        });
        for (c, val) in curr.iter_mut().enumerate() {
            *val += prefix[r][c];
        }
        prefix.push(curr);
    }
    if prefix[rows][cols] < 2 {
        return 0;
    }
    let mut left = 1;
    let mut right = rows.min(cols);
    while left < right {
        let mid = left + (1 + right - left) / 2;
        if check(&prefix, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left.pow(2) as i32
}

fn check(prefix: &[Vec<i32>], mid: usize) -> bool {
    let [rows, cols] = get_dimensions(&prefix);
    let [mut minr, mut minc] = [rows, cols];
    let [mut maxr, mut maxc] = [0, 0];
    let mut count = 0;
    for r in mid..rows {
        for c in mid..cols {
            let area =
                prefix[r][c] - prefix[r][c - mid] - prefix[r - mid][c] + prefix[r - mid][c - mid];
            if area == (mid as i32).pow(2) {
                count += 1;
                minr = minr.min(r);
                minc = minc.min(c);
                maxr = maxr.max(r);
                maxc = maxc.max(c);
            }
        }
    }
    count >= 2 && (maxr.abs_diff(minr) >= mid || maxc.abs_diff(minc) >= mid)
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
        assert_eq!(max_area(&[&[1, 1, 1, 0], &[1, 1, 1, 1], &[0, 0, 1, 1]]), 4)
    }

    #[test]
    fn test() {}
}

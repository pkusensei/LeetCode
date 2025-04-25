mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_operations_to_write_y(grid: &[&[i32]]) -> i32 {
    let n = grid.len();
    let [mut fin, mut fout] = [vec![0; 3], vec![0; 3]];
    let [mut cin, mut cout] = [0, 0];
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if in_y(n, r, c) {
                fin[v as usize] += 1;
                cin += 1;
            } else {
                fout[v as usize] += 1;
                cout += 1;
            }
        }
    }
    let mut res = (n as i32).pow(2);
    for y in 0..3 {
        let mut curr = cin - fin[y];
        let mut max = 0;
        for out in (0..3).filter(|&v| v != y) {
            max = max.max(fout[out]);
        }
        curr += cout - max;
        res = res.min(curr);
    }
    res
}

const fn in_y(n: usize, r: usize, c: usize) -> bool {
    if r <= n / 2 {
        r == c || r + c + 1 == n
    } else {
        c == n / 2
    }
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
            minimum_operations_to_write_y(&[&[1, 2, 2], &[1, 1, 0], &[0, 1, 0]]),
            3
        );
    }

    #[test]
    fn test() {}
}

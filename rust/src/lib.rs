mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_points(points: &[&[i32]]) -> i64 {
    let [rows, cols] = get_dimensions(points);
    if rows == 1 {
        return i64::from(*points[0].iter().max().unwrap());
    }
    if cols == 1 {
        return points.iter().map(|r| i64::from(r[0])).sum();
    }
    let mut prev: Vec<_> = points[0].iter().map(|&v| i64::from(v)).collect();
    for row in points.iter().skip(1) {
        let mut left = vec![prev[0]];
        for col in 1..cols {
            left.push(prev[col].max(left[col - 1] - 1));
        }
        let mut right = vec![prev[cols - 1]];
        for col in (0..cols - 1).rev() {
            right.push(prev[col].max(right.last().unwrap() - 1));
        }
        right.reverse();
        let curr = row
            .iter()
            .zip(left.into_iter().zip(right))
            .map(|(&num, (lv, rv))| i64::from(num) + lv.max(rv))
            .collect();
        prev = curr;
    }
    prev.into_iter().max().unwrap()
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
        assert_eq!(max_points(&[&[1, 2, 3], &[1, 5, 1], &[3, 1, 1]]), 9);
        assert_eq!(max_points(&[&[1, 5], &[2, 3], &[4, 2]]), 11);
    }

    #[test]
    fn test() {}
}

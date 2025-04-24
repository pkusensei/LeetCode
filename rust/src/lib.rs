mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_pairs(mut points: Vec<[i32; 2]>) -> i32 {
    points.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let mut res = 0;
    for (i, p1) in points.iter().enumerate() {
        let y1 = p1[1];
        let mut max_y = i32::MIN;
        for p2 in points.iter().skip(1 + i) {
            let y2 = p2[1];
            if y1 >= y2 && y2 > max_y {
                res += 1;
                max_y = y2;
            }
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
        assert_eq!(number_of_pairs(vec![[1, 1], [2, 2], [3, 3]]), 0);
        assert_eq!(number_of_pairs(vec![[6, 2], [4, 4], [2, 6]]), 2);
        assert_eq!(number_of_pairs(vec![[3, 1], [1, 3], [1, 1]]), 2);
    }

    #[test]
    fn test() {}
}

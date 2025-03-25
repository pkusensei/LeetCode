mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_valid_cuts(_n: i32, rectangles: &[[i32; 4]]) -> bool {
    let mut xspans = vec![];
    let mut yspans = vec![];
    for rec in rectangles.iter() {
        let [x1, y1, x2, y2] = rec[..] else {
            unreachable!()
        };
        xspans.push([x1, x2]);
        yspans.push([y1, y2]);
    }
    xspans.sort_unstable();
    yspans.sort_unstable();
    check(&xspans) || check(&yspans)
}

fn check(spans: &[[i32; 2]]) -> bool {
    let mut curr_end = spans[0][1];
    let mut count = 0;
    for sp in spans[1..].iter() {
        let [start, end] = *sp;
        if start >= curr_end {
            count += 1;
        }
        curr_end = curr_end.max(end);
        if count >= 2 {
            break;
        }
    }
    count >= 2
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
        // assert!(check_valid_cuts(
        //     5,
        //     &[[1, 0, 5, 2], [0, 2, 2, 4], [3, 2, 5, 3], [0, 4, 4, 5]]
        // ));
        // assert!(check_valid_cuts(
        //     4,
        //     &[[0, 0, 1, 1], [2, 0, 3, 4], [0, 2, 2, 3], [3, 0, 4, 3]]
        // ));
        assert!(!check_valid_cuts(
            4,
            &[
                [0, 2, 2, 4],
                [1, 0, 3, 2],
                [2, 2, 3, 4],
                [3, 0, 4, 2],
                [3, 2, 4, 4]
            ]
        ));
    }

    #[test]
    fn test() {
        assert!(!check_valid_cuts(
            4,
            &[
                [0, 0, 1, 4],
                [1, 0, 2, 3],
                [2, 0, 3, 3],
                [3, 0, 4, 3],
                [1, 3, 4, 4]
            ]
        ));
    }
}

mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
    let [mut xs, mut ys] = [vec![], vec![]];
    for r in &rectangles {
        let [x1, y1, x2, y2] = r[..] else {
            unreachable!()
        };
        xs.push([x1, x2]);
        ys.push([y1, y2]);
    }
    solve(xs) || solve(ys)
}

fn solve(mut arr: Vec<[i32; 2]>) -> bool {
    arr.sort_unstable();
    let mut count = 0;
    let mut end = arr[0][1];
    for &[x1, x2] in &arr[1..] {
        count += i32::from(x1 >= end);
        end = end.max(x2);
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
    fn basics() {}

    #[test]
    fn test() {}
}

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
    let mut recs = vec![vec![]; 101];
    for rect in rectangles.iter() {
        let [x, y] = rect[..] else { unreachable!() };
        recs[y as usize].push(x);
    }
    for r in recs.iter_mut() {
        r.sort_unstable_by(|a, b| b.cmp(a));
    }
    points
        .iter()
        .map(|p| {
            let [x, y] = p[..] else { unreachable!() };
            recs[y as usize..]
                .iter()
                .map(|r| r.partition_point(|&v| v <= x))
                .sum::<usize>() as i32
        })
        .collect()
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

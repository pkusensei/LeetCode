mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_white_tiles(tiles: &mut [[i32; 2]], carpet_len: i32) -> i32 {
    tiles.sort_unstable();
    let prefix = tiles.iter().fold(vec![], |mut acc, t| {
        acc.push(t[1] - t[0] + 1 + acc.last().unwrap_or(&0));
        acc
    });
    let mut res = 0;
    for (idx, tile) in tiles.iter().enumerate() {
        let [left, right] = tile[..] else {
            unreachable!()
        };
        // Put on left
        let i = tiles.partition_point(|sp| sp[0] < left + carpet_len);
        if i == 1 + idx {
            res = res.max((1 + right - left).min(carpet_len));
        } else {
            let mut curr = prefix[i - 2] - if idx > 0 { prefix[idx - 1] } else { 0 };
            curr += (left + carpet_len).min(1 + tiles[i - 1][1]) - tiles[i - 1][0];
            res = res.max(curr);
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
        assert_eq!(
            maximum_white_tiles(&mut [[1, 5], [10, 11], [12, 18], [20, 25], [30, 32]], 10),
            9
        );
        assert_eq!(maximum_white_tiles(&mut [[10, 11], [1, 1]], 2), 2);
    }

    #[test]
    fn test() {}
}

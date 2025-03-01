mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
    let mut res = vec![];
    let m = s.len();
    'outer: for i in 0..m {
        let [mut row, mut col] = start_pos[..] else {
            unreachable!()
        };
        let mut idx = i;
        while (0..n).contains(&row) && (0..n).contains(&col) {
            let Some(b) = s.as_bytes().get(idx) else {
                res.push((m - i) as i32);
                continue 'outer;
            };
            idx += 1;
            match b {
                b'L' => col -= 1,
                b'R' => col += 1,
                b'U' => row -= 1,
                _ => row += 1,
            }
        }
        res.push((idx - i - 1) as i32);
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
    fn basics() {}

    #[test]
    fn test() {}
}

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_collisions(directions: &str) -> i32 {
    let mut stack = vec![];
    let mut res = 0;
    for b in directions.bytes() {
        match b {
            b'S' => {
                while stack.last().is_some_and(|&v| v == b'R') {
                    stack.pop();
                    res += 1;
                }
                stack.push(b'S');
            }
            b'L' => match stack.last() {
                Some(b'R') => {
                    res += 1;
                    while stack.last().is_some_and(|&v| v == b'R') {
                        stack.pop();
                        res += 1;
                    }
                    stack.push(b'S');
                }
                Some(b'S') => res += 1,
                _ => (),
            },
            _ => stack.push(b),
        }
    }
    res
}

pub fn towards_middle(s: &str) -> i32 {
    let mut s = s.trim_start_matches('L');
    s = s.trim_end_matches('R');
    s.bytes().map(|b| i32::from(b != b'S')).sum()
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
        assert_eq!(count_collisions("RLRSLL"), 5);
        assert_eq!(count_collisions("LLRR"), 0);

        assert_eq!(towards_middle("RLRSLL"), 5);
        assert_eq!(towards_middle("LLRR"), 0);
    }

    #[test]
    fn test() {}
}

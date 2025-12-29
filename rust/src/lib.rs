mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn pyramid_transition(bottom: &str, allowed: &[&str]) -> bool {
    let rules = allowed.iter().fold([[0; 6]; 6], |mut acc, s| {
        let [a, b, c] = s.as_bytes()[..] else {
            unreachable!()
        };
        let a = usize::from(a - b'A');
        let b = usize::from(b - b'A');
        acc[a][b] |= 1 << (c - b'A');
        acc
    });
    let line = bottom.bytes().map(|b| b - b'A').collect_vec();
    backtrack(&line, &rules, &mut vec![])
}

fn backtrack(line: &[u8], rules: &[[u8; 6]; 6], curr: &mut Vec<u8>) -> bool {
    if line.len() <= 1 {
        if curr.is_empty() {
            return true;
        }
        return backtrack(curr, rules, &mut vec![]);
    }
    let a = usize::from(line[0]);
    let b = usize::from(line[1]);
    for bit in 0..6 {
        if rules[a][b] & (1 << bit) > 0 {
            curr.push(bit as u8);
            if backtrack(&line[1..], rules, curr) {
                return true;
            }
            curr.pop();
        }
    }
    false
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
        assert!(pyramid_transition("BCD", &["BCC", "CDE", "CEA", "FFF"]))
    }

    #[test]
    fn test() {}
}

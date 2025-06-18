mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn calculate_score(instructions: Vec<String>, values: Vec<i32>) -> i64 {
    use std::collections::HashSet;
    let mut res = 0;
    let mut idx = 0;
    let mut set = HashSet::new();
    while let Some(s) = usize::try_from(idx).ok().and_then(|i| instructions.get(i)) {
        if !set.insert(idx) {
            break;
        }
        match s.as_str() {
            "add" => {
                res += i64::from(values[idx as usize]);
                idx += 1;
            }
            "jump" => idx += values[idx as usize],
            _ => unreachable!(),
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
    fn basics() {}

    #[test]
    fn test() {}
}

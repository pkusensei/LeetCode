mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
    let n = skills.len();
    if n <= k as usize {
        return skills
            .iter()
            .enumerate()
            .max_by_key(|&(_i, &v)| v)
            .map(|(i, _)| i as i32)
            .unwrap();
    }
    let mut curr = 0;
    let mut count = 0;
    for (i, &num) in skills.iter().enumerate().skip(1) {
        if skills[curr] >= num {
            count += 1;
        } else {
            curr = i;
            count = 1;
        }
        if count == k {
            return curr as i32;
        }
    }
    curr as i32
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

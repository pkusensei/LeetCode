mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_time(skill: &[i32], mana: &[i32]) -> i64 {
    let n = skill.len();
    let mut done = vec![0; 1 + n];
    for &m in mana.iter() {
        for (i, &s) in skill.iter().enumerate() {
            // [1+i] starts woking on this potion when
            // [i] finishes on it
            // OR
            // [1+i] finishes prev potion
            done[1 + i] = done[1 + i].max(done[i]) + i64::from(m * s);
        }
        for (i, &s) in skill.iter().enumerate().rev() {
            // Backtrack to find free time of [i]
            done[i] = done[1 + i] - i64::from(s * m);
        }
    }
    done[n]
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
        assert_eq!(min_time(&[1, 5, 2, 4], &[5, 1, 4, 2]), 110);
        assert_eq!(min_time(&[1, 1, 1], &[1, 1, 1]), 5);
        assert_eq!(min_time(&[1, 2, 3, 4], &[1, 2]), 21);
    }

    #[test]
    fn test() {}
}

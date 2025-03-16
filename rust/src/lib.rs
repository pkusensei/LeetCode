mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
    let mut map = std::collections::HashMap::new();
    let mut res = 0;
    for &t in tasks.iter() {
        res += 1;
        if let Some(&prev) = map.get(&t) {
            res = res.max(prev + i64::from(space) + 1);
        }
        map.insert(t, res);
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

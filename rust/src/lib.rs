mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_compatibility_sum(students: &[&[i32]], mentors: &[&[i32]]) -> i32 {
    let [m, n] = get_dimensions(students);
    let smasks: Vec<_> = students
        .iter()
        .map(|s| s.iter().fold(0, |acc, v| (acc << 1) | (v & 1)))
        .collect();
    let mmasks: Vec<_> = mentors
        .iter()
        .map(|m| m.iter().fold(0, |acc, v| (acc << 1) | (v & 1)))
        .collect();
    let mut memo = vec![vec![-1; 1 << m]; m];
    dfs(&smasks, &mmasks, n as i32, 0, 0, &mut memo)
}

fn dfs(
    smasks: &[i32],
    mmasks: &[i32],
    n: i32,
    idx: usize,
    mask: usize,
    memo: &mut [Vec<i32>],
) -> i32 {
    if idx >= mmasks.len() {
        return 0;
    }
    if memo[idx][mask] > -1 {
        return memo[idx][mask];
    }
    let mut res = 0;
    for (si, stu) in smasks.iter().enumerate() {
        if mask & (1 << si) > 0 {
            continue;
        }
        let curr: i32 = n - (stu ^ mmasks[idx]).count_ones() as i32;
        res = res.max(curr + dfs(smasks, mmasks, n, 1 + idx, mask | (1 << si), memo));
    }
    memo[idx][mask] = res;
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
            max_compatibility_sum(
                &[&[1, 1, 0], &[1, 0, 1], &[0, 0, 1]],
                &[&[1, 0, 0], &[0, 0, 1], &[1, 1, 0]]
            ),
            8
        );
        assert_eq!(
            max_compatibility_sum(&[&[0, 0], &[0, 0], &[0, 0]], &[&[1, 1], &[1, 1], &[1, 1]]),
            0
        )
    }

    #[test]
    fn test() {}
}

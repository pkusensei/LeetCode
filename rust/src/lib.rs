mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn possibly_equals(s1: &str, s2: &str) -> bool {
    let (n1, n2) = (s1.len(), s2.len());
    let mut memo = vec![vec![[None; 2000]; 1 + n2]; 1 + n1];
    dfs(s1.as_bytes(), s2.as_bytes(), 0, 0, 0, &mut memo)
}

fn dfs(
    s1: &[u8],
    s2: &[u8],
    i1: usize,
    i2: usize,
    diff: i32,
    memo: &mut [Vec<[Option<bool>; 2000]>],
) -> bool {
    let (n1, n2) = (s1.len(), s2.len());
    if i1 == n1 && i2 == n2 {
        return diff == 0;
    }
    if let Some(v) = memo[i1][i2][(1000 + diff) as usize] {
        return v;
    }
    if s1.get(i1).zip(s2.get(i2)).is_some_and(|(a, b)| a == b)
        && diff == 0
        && dfs(s1, s2, 1 + i1, 1 + i2, 0, memo)
    {
        memo[i1][i2][(1000 + diff) as usize] = Some(true);
        return true;
    }
    if s1.get(i1).is_some_and(|b| b.is_ascii_alphabetic())
        && diff > 0
        && dfs(s1, s2, 1 + i1, i2, diff - 1, memo)
    {
        memo[i1][i2][(1000 + diff) as usize] = Some(true);
        return true;
    }
    if s2.get(i2).is_some_and(|b| b.is_ascii_alphabetic())
        && diff < 0
        && dfs(s1, s2, i1, 1 + i2, diff + 1, memo)
    {
        memo[i1][i2][(1000 + diff) as usize] = Some(true);
        return true;
    }
    let mut num = 0;
    let mut idx = i1;
    while s1.get(idx).is_some_and(|b| b.is_ascii_digit()) {
        num = 10 * num + i32::from(s1[idx] - b'0');
        if dfs(s1, s2, 1 + idx, i2, diff - num, memo) {
            memo[i1][i2][(1000 + diff) as usize] = Some(true);
            return true;
        }
        idx += 1;
    }
    num = 0;
    idx = i2;
    while s2.get(idx).is_some_and(|b| b.is_ascii_digit()) {
        num = 10 * num + i32::from(s2[idx] - b'0');
        if dfs(s1, s2, i1, 1 + idx, diff + num, memo) {
            memo[i1][i2][(1000 + diff) as usize] = Some(true);
            return true;
        }
        idx += 1;
    }
    memo[i1][i2][(1000 + diff) as usize] = Some(false);
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
        assert!(possibly_equals("internationalization", "i18n"));
        assert!(possibly_equals("l123e", "44"));
        assert!(!possibly_equals("a5b", "c5b"));
    }

    #[test]
    fn test() {
        assert!(!possibly_equals("x94", "x14"));
    }
}

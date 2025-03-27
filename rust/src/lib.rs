mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut map = std::collections::HashMap::<i32, [i32; 2]>::new();
    let n = s.len();
    let mut idx = 0;
    while idx < n {
        for len in 1..=30 {
            if idx + len <= n {
                let num = i32::from_str_radix(&s[idx..idx + len], 2).unwrap();
                if let Some(v) = map.get_mut(&num) {
                    if v[1] - v[0] + 1 > len as i32 {
                        *v = [idx as i32, (idx + len - 1) as i32];
                    }
                } else {
                    map.insert(num, [idx as i32, (idx + len - 1) as i32]);
                }
            }
        }
        idx += 1;
    }
    queries
        .iter()
        .map(|q| {
            let v = q[0] ^ q[1];
            map.get(&v).map(|v| v.to_vec()).unwrap_or(vec![-1, -1])
        })
        .collect()
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

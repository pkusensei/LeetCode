mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut map = std::collections::HashMap::new();
    for m in matches.iter() {
        let [a, b] = m[..] else { unreachable!() };
        map.entry(a).or_insert(0);
        *map.entry(b).or_insert(0) += 1;
    }
    let [mut win, mut one] = [vec![], vec![]];
    for (&k, &v) in map.iter() {
        match v {
            0 => win.push(k),
            1 => one.push(k),
            _ => (),
        }
    }
    win.sort_unstable();
    one.sort_unstable();
    vec![win, one]
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

mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn find_high_access_employees(access_times: Vec<Vec<String>>) -> Vec<String> {
    let mut map = HashMap::new();
    for a in access_times.iter() {
        let (name, time) = parse(a);
        map.entry(name).or_insert(vec![]).push(time);
    }
    let mut res = vec![];
    for (k, v) in map.iter_mut() {
        v.sort_unstable();
        for (left, a) in v.iter().enumerate() {
            let right = v.partition_point(|&x| x < a + 60);
            if right - left >= 3 {
                res.push(k.to_string());
                break;
            }
        }
    }
    res
}

fn parse(line: &[String]) -> (&str, i32) {
    let name = line[0].as_str();
    let h = line[1][..2].parse::<i32>().unwrap_or_default();
    let m = line[1][2..].parse::<i32>().unwrap_or_default();
    (name, h * 60 + m)
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

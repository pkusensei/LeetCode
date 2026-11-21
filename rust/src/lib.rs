mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn assign_umbrella(rain: Vec<i32>) -> Vec<String> {
    let n = rain.len();
    let mut res = vec![];
    let [mut ali, mut bob] = [0, 0];
    for (i, &v) in rain.iter().enumerate() {
        if v == 1 {
            if i < n - 1 && rain[1 + i] == 1 && ali <= bob {
                res.push("Alice".to_string());
                ali += 1;
            } else {
                res.push("Bob".to_string());
                bob += 1;
            }
        } else {
            res.push("None".to_string());
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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

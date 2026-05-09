mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn score_validator(events: Vec<String>) -> Vec<i32> {
    let [mut score, mut counter] = [0, 0];
    for e in events.iter() {
        match e.as_str() {
            "0" | "1" | "2" | "3" | "4" | "6" => score += e.parse::<i32>().unwrap_or(0),
            "W" => counter += 1,
            "WD" | "NB" => score += 1,
            _ => (),
        }
        if counter == 10 {
            break;
        }
    }
    [score, counter].to_vec()
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

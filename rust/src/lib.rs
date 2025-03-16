mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
    let r_count = ranks.iter().fold([0; 14], |mut acc, &num| {
        acc[num as usize] += 1;
        acc
    });
    let s_count = suits
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        });
    if s_count.len() == 1 {
        "Flush".into()
    } else {
        match r_count.into_iter().max() {
            Some(v) if v >= 3 => "Three of a Kind".into(),
            Some(2) => "Pair".into(),
            _ => "High Card".into(),
        }
    }
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

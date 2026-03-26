mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let [rows, cols] = get_dimensions(&strs);
    if rows <= 1 {
        return 0;
    }
    let mut states = vec![State::None; rows - 1];
    let mut res = 0;
    'out: for c in 0..cols {
        let mut curr = states.clone();
        for (r, w) in strs.windows(2).enumerate() {
            match w[0].as_bytes()[c].cmp(&w[1].as_bytes()[c]) {
                std::cmp::Ordering::Less => curr[r] = curr[r].max(State::Inc),
                std::cmp::Ordering::Equal => curr[r] = curr[r].max(State::Equal),
                std::cmp::Ordering::Greater => {
                    if matches!(curr[r], State::None | State::Equal) {
                        res += 1;
                        continue 'out;
                    }
                }
            }
        }
        states = curr;
    }
    res
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    None,
    Equal,
    Inc,
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

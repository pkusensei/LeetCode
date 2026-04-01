mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn survived_robots_healths(
    positions: Vec<i32>,
    healths: Vec<i32>,
    directions: String,
) -> Vec<i32> {
    use itertools::{Itertools, izip};
    // (pos, health, dir, idx)
    let robs = izip!(positions.iter(), healths.iter(), directions.bytes())
        .enumerate()
        .map(|(i, (p, h, b))| (*p, *h, b, i))
        .sorted_unstable()
        .collect_vec();
    // (pos, health, dir, idx)
    let mut st = Vec::<(i32, i32, u8, usize)>::new();
    'out: for rob in robs {
        let mut rob = rob;
        while let Some(top) = st.last()
            && top.2 == b'R'
            && rob.2 == b'L'
        {
            let mut top = *top;
            st.pop();
            match top.1.cmp(&rob.1) {
                std::cmp::Ordering::Less => rob.1 -= 1,
                std::cmp::Ordering::Equal => continue 'out,
                std::cmp::Ordering::Greater => {
                    top.1 -= 1;
                    rob = top;
                }
            };
        }
        st.push(rob);
    }
    st.into_iter()
        .sorted_unstable_by_key(|rob| rob.3)
        .map(|r| r.1)
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

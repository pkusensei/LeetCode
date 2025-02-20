mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_weak_characters(properties: &mut [[i32; 2]]) -> i32 {
    // inc by attack, dec by defense
    properties.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let mut res = 0;
    let mut stack = vec![];
    for p in properties.iter() {
        let curr = p[1];
        while stack.last().is_some_and(|&v| v < curr) {
            stack.pop(); // Pop lower atack AND lower defense
            res += 1;
        }
        stack.push(curr);
    }
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
        assert_eq!(number_of_weak_characters(&mut [[5, 5], [6, 3], [3, 6]]), 0);
        assert_eq!(number_of_weak_characters(&mut [[2, 2], [3, 3]]), 1);
        assert_eq!(number_of_weak_characters(&mut [[1, 5], [10, 4], [4, 3]]), 1);
    }

    #[test]
    fn test() {}
}

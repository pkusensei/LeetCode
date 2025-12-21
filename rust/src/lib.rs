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
    let mut keep: Vec<usize> = vec![];
    for c in 0..cols {
        let mut keep_curr = true;
        for r in 1..rows {
            if strs[r - 1].as_bytes()[c] > strs[r].as_bytes()[c] {
                keep_curr = false;
                for &left in keep.iter().rev() {
                    if strs[r - 1].as_bytes()[left] < strs[r].as_bytes()[left] {
                        keep_curr = true;
                        break;
                    }
                }
                if !keep_curr {
                    break;
                }
            }
        }
        if keep_curr {
            keep.push(c);
        }
    }
    (cols - keep.len()) as i32
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

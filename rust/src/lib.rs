mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn lemonade_change(bills: &[i32]) -> bool {
    let mut freq = [0; 2];
    for num in bills {
        match num {
            5 => freq[0] += 1,
            10 => {
                freq[1] += 1;
                freq[0] -= 1;
                if freq[0] < 0 {
                    return false;
                }
            }
            20 => {
                if freq[1] > 0 {
                    freq[1] -= 1;
                    freq[0] -= 1;
                } else {
                    freq[0] -= 3
                }
                if freq.iter().any(|&v| v < 0) {
                    return false;
                }
            }
            _ => (),
        }
    }
    true
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
    fn test() {
        assert!(lemonade_change(&[5, 5, 10, 10, 5, 20, 5, 10, 5, 5]))
    }
}

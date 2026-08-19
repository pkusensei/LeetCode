mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for re in reserved_seats.iter() {
        let [row, seat] = re[..] else { unreachable!() };
        let v = map.entry(row).or_insert(0b111_1111_111);
        *v ^= 1 << (seat - 1);
    }
    let mut res = (n - map.len() as i32) * 2;
    for v in map.into_values() {
        const TWO: i32 = 0b011_1111_110;
        const A: i32 = 0b000_1111_000;
        const B: i32 = 0b011_1100_000;
        const C: i32 = 0b000_0011_110;
        if v & TWO == TWO {
            res += 2
        } else {
            for x in [A, B, C] {
                if v & x == x {
                    res += 1;
                    break;
                }
            }
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

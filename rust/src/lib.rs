mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
    let mut prev = 0;
    let mut res = 0.0;
    for bra in brackets.iter() {
        let [up, per] = bra[..] else { unreachable!() };
        res += f64::from((income.min(up) - prev) * per) / 100.0;
        prev = up;
        if up >= income {
            break;
        }
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
    fn basics() {}

    #[test]
    fn test() {}
}

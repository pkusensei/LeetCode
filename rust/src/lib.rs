mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_lines(mut stock_prices: Vec<Vec<i32>>) -> i32 {
    let n = stock_prices.len();
    if n <= 2 {
        return (n as i32 - 1).max(0);
    }
    stock_prices.sort_unstable_by_key(|s| s[0]);
    let [x0, y0] = stock_prices[0][..] else {
        unreachable!()
    };
    let [x1, y1] = stock_prices[1][..] else {
        unreachable!()
    };
    let mut dx = i64::from(x1 - x0);
    let mut dy = i64::from(y1 - y0);
    let mut res = 1;
    for w in stock_prices.windows(2).skip(1) {
        let [x0, y0] = w[0][..] else { unreachable!() };
        let [x1, y1] = w[1][..] else { unreachable!() };
        let cdx = i64::from(x1 - x0);
        let cdy = i64::from(y1 - y0);
        if dy * cdx != dx * cdy {
            res += 1;
            dx = cdx;
            dy = cdy;
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

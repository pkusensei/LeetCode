mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost_set_time(
    start_at: i32,
    move_cost: i32,
    push_cost: i32,
    target_seconds: i32,
) -> i32 {
    let m = target_seconds / 60;
    let s = target_seconds % 60;
    if m >= 100 {
        return cost(start_at, move_cost, push_cost, m - 1, 60 + s);
    }
    let res = cost(start_at, move_cost, push_cost, m, s);
    if m > 0 && s <= 39 {
        res.min(cost(start_at, move_cost, push_cost, m - 1, 60 + s))
    } else {
        res
    }
}

const fn cost(start: i32, m_cost: i32, p_cost: i32, m: i32, s: i32) -> i32 {
    let mut num = 100 * m + s;
    let mut res = 0;
    let mut digit = num % 10;
    while num > 0 {
        res += p_cost;
        if digit != num % 10 {
            res += m_cost;
            digit = num % 10;
        }
        num /= 10;
    }
    if start != digit {
        res + m_cost
    } else {
        res
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

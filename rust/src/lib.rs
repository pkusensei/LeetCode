mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn closest_cost(base_costs: &[i32], topping_costs: &[i32], target: i32) -> i32 {
    let mut res: i32 = -10_000;
    for &base in base_costs.iter() {
        backtrack(topping_costs, target, base, &mut res);
    }
    res
}

fn backtrack(tops: &[i32], target: i32, curr: i32, res: &mut i32) {
    if curr - target > (target - *res).abs() {
        return;
    }
    match tops {
        [] => match curr.abs_diff(target).cmp(&res.abs_diff(target)) {
            std::cmp::Ordering::Less => *res = curr,
            std::cmp::Ordering::Equal => *res = (*res).min(curr),
            std::cmp::Ordering::Greater => (),
        },
        [head, tail @ ..] => {
            backtrack(tail, target, curr, res); // skip
            backtrack(tail, target, curr + head, res); // 1 take
            backtrack(tail, target, curr + 2 * head, res); // 2 takes
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(
    start_pos: Vec<i32>,
    home_pos: Vec<i32>,
    row_costs: Vec<i32>,
    col_costs: Vec<i32>,
) -> i32 {
    let [r_start, c_start] = [0, 1].map(|i| start_pos[i] as usize);
    let [r_goal, c_goal] = [0, 1].map(|i| home_pos[i] as usize);
    match [r_start == r_goal, c_start == c_goal] {
        [true, true] => 0,
        [true, false] => col_costs[(1 + c_start).min(c_goal)..c_start.max(1 + c_goal)]
            .iter()
            .sum(),
        [false, true] => row_costs[(1 + r_start).min(r_goal)..r_start.max(1 + r_goal)]
            .iter()
            .sum(),
        _ => {
            row_costs[(1 + r_start).min(r_goal)..r_start.max(1 + r_goal)]
                .iter()
                .sum::<i32>()
                + col_costs[(1 + c_start).min(c_goal)..c_start.max(1 + c_goal)]
                    .iter()
                    .sum::<i32>()
        }
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

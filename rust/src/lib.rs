mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn maximum_robots(charge_times: &[i32], running_costs: &[i32], budget: i64) -> i32 {
    let mut res = 0;
    let mut charges = BTreeMap::new();
    let mut sum_cost = 0;
    let mut left = 0;
    for (right, (&ch, &co)) in charge_times.iter().zip(running_costs).enumerate() {
        let [ch, co] = [ch, co].map(i64::from);
        *charges.entry(ch).or_insert(0) += 1;
        sum_cost += co;
        while cost(&charges, sum_cost, (right + 1 - left) as _) > budget {
            sum_cost -= i64::from(running_costs[left]);
            let del_ch = i64::from(charge_times[left]);
            charges.entry(del_ch).and_modify(|v| *v -= 1);
            if charges[&del_ch] == 0 {
                charges.remove(&del_ch);
            }
            left += 1;
        }
        if left <= right {
            res = res.max(right + 1 - left);
        }
    }
    res as _
}

fn cost(charges: &BTreeMap<i64, i32>, sum_cost: i64, k: i64) -> i64 {
    charges.last_key_value().map(|(&k, _)| k).unwrap_or(0) + k * sum_cost
}

pub fn with_deque(charge_times: &[i32], running_costs: &[i32], budget: i64) -> i32 {
    let mut res = 0;
    let mut charges = VecDeque::new();
    let mut sum_cost = 0;
    let mut left = 0;
    for (right, (&ch, &co)) in charge_times.iter().zip(running_costs.iter()).enumerate() {
        let [ch, co] = [ch, co].map(i64::from);
        sum_cost += co;
        while charges
            .back()
            .is_some_and(|&v| i64::from(charge_times[v]) <= ch)
        {
            charges.pop_back();
        }
        charges.push_back(right);
        while let Some(&front) = charges.front() {
            if i64::from(charge_times[front]) + (right + 1 - left) as i64 * sum_cost > budget {
                if front == left {
                    charges.pop_front();
                }
                sum_cost -= i64::from(running_costs[left]);
                left += 1;
            } else {
                break;
            }
        }
        res = res.max(right + 1 - left);
    }
    res as _
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
        assert_eq!(maximum_robots(&[3, 6, 1, 3, 4], &[2, 1, 3, 4, 5], 25), 3);
        assert_eq!(maximum_robots(&[11, 12, 19], &[10, 8, 7], 19), 0);

        assert_eq!(with_deque(&[3, 6, 1, 3, 4], &[2, 1, 3, 4, 5], 25), 3);
        assert_eq!(with_deque(&[11, 12, 19], &[10, 8, 7], 19), 0);
    }

    #[test]
    fn test() {}
}

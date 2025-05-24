mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn find_x_sum1(nums: &[i32], k: i32, x: i32) -> Vec<i64> {
    let [k, x] = [k, x].map(|v| v as usize);
    let mut res = vec![];
    let mut freq = HashMap::new();

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    struct Element {
        count: i64,
        value: i64,
    }
    let mut top = BTreeSet::new();
    let mut bot = BTreeSet::new();
    let mut x_sum: i64 = 0;
    for (idx, &num) in nums.iter().enumerate() {
        let count = *freq.get(&num).unwrap_or(&0);
        // Remove existing entry if present
        if count > 0 {
            let elem = Element {
                count: i64::from(count),
                value: i64::from(num),
            };
            // If in bot, no need to adjust running_sum
            // Otherwise..
            if !bot.remove(&elem) {
                top.remove(&elem);
                x_sum -= i64::from(count) * i64::from(num);
            }
        }
        // Insert with new count
        let new_count = count + 1;
        freq.insert(num, new_count);
        let new_elem = Element {
            count: i64::from(new_count),
            value: i64::from(num),
        };
        top.insert(new_elem);
        x_sum += i64::from(new_count) * i64::from(num);
        // Maintain top x elements
        if top.len() > x {
            if let Some(&elem) = top.iter().next() {
                x_sum -= elem.count * elem.value;
                top.remove(&elem);
                bot.insert(elem);
            }
        }
        // Handle window sliding
        if idx >= k {
            let old_num = nums[idx - k];
            let old_count = *freq.get(&old_num).unwrap();
            let old_elem = Element {
                count: i64::from(old_count),
                value: i64::from(old_num),
            };
            if !bot.remove(&old_elem) {
                // Was in bot, no need to adjust running_sum
                top.remove(&old_elem);
                x_sum -= i64::from(old_count) * i64::from(old_num);
            }
            // Decrement count
            if old_count > 1 {
                freq.insert(old_num, old_count - 1);
                let new_elem = Element {
                    count: i64::from(old_count - 1),
                    value: i64::from(old_num),
                };
                bot.insert(new_elem);
            } else {
                freq.remove(&old_num);
            }
            // Maintain top x elements
            if top.len() < x {
                if let Some(&elem) = bot.iter().next_back() {
                    x_sum += elem.count * elem.value;
                    bot.remove(&elem);
                    top.insert(elem);
                }
            }
        }
        // Record result
        if idx + 1 >= k {
            res.push(x_sum);
        }
    }
    res
}

pub fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i64> {
    let [k, x] = [k, x].map(|v| v as usize);
    let mut num_freqs = nums[..k].iter().fold(HashMap::new(), |mut acc, &v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    });
    let mut top_freqs = BTreeMap::<_, BTreeSet<_>>::new();
    let mut candids = BTreeMap::<_, BTreeSet<_>>::new();
    let mut window = HashSet::new();
    for (&num, &c) in num_freqs.iter() {
        candids.entry(c).or_default().insert(num);
    }
    balance(x, &mut top_freqs, &mut candids, &mut window);
    let mut res = vec![x_sum(&top_freqs)];
    for (idx, &num) in nums.iter().enumerate().skip(k) {
        let v = num_freqs.entry(num).or_insert(0);
        if window.remove(&num) {
            // num is already in top_x
            top_freqs.entry(*v).or_default().remove(&num);
        } else {
            candids.entry(*v).or_default().remove(&num);
        }
        *v += 1;
        candids.entry(*v).or_default().insert(num);

        let left = nums[idx - k];
        let v = num_freqs.entry(left).or_insert(0);
        if window.remove(&left) {
            top_freqs.entry(*v).or_default().remove(&left);
        } else {
            candids.entry(*v).or_default().remove(&left);
        }
        *v -= 1;
        if *v > 0 {
            candids.entry(*v).or_default().insert(left);
        }
        if *v == 0 {
            num_freqs.remove(&left);
        }
        balance(x, &mut top_freqs, &mut candids, &mut window);
        res.push(x_sum(&top_freqs));
    }
    res
}

fn balance(
    x: usize,
    top_freqs: &mut BTreeMap<i32, BTreeSet<i32>>,
    candids: &mut BTreeMap<i32, BTreeSet<i32>>,
    window: &mut HashSet<i32>,
) {
    while candids
        .last_key_value()
        .zip(top_freqs.first_key_value())
        .is_some_and(|(a, b)| a.0 >= b.0)
    {
        let (c, nums) = top_freqs.pop_first().unwrap();
        for num in &nums {
            window.remove(num);
        }
        candids.entry(c).or_default().extend(nums);
    }
    while window.len() > x {
        let (c, mut nums) = top_freqs.pop_first().unwrap();
        while let Some(num) = nums.pop_first() {
            window.remove(&num);
            candids.entry(c).or_default().insert(num);
            if window.len() <= x {
                break;
            }
        }
        if !nums.is_empty() {
            top_freqs.insert(c, nums);
        }
    }
    while window.len() < x {
        let Some((c, mut nums)) = candids.pop_last() else {
            break;
        };
        while let Some(num) = nums.pop_last() {
            window.insert(num);
            top_freqs.entry(c).or_default().insert(num);
            if window.len() >= x {
                break;
            }
        }
        if !nums.is_empty() {
            candids.insert(c, nums);
        }
    }
}

fn x_sum(top: &BTreeMap<i32, BTreeSet<i32>>) -> i64 {
    top.iter()
        .flat_map(|(&c, set)| set.iter().map(move |&num| i64::from(num) * i64::from(c)))
        .sum()
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
        assert_eq!(find_x_sum(&[1, 1, 2, 2, 3, 4, 2, 3], 6, 2), [6, 10, 12]);
        assert_eq!(find_x_sum(&[3, 8, 7, 8, 7, 5], 2, 2), [11, 15, 15, 15, 12]);

        assert_eq!(find_x_sum1(&[1, 1, 2, 2, 3, 4, 2, 3], 6, 2), [6, 10, 12]);
        assert_eq!(find_x_sum1(&[3, 8, 7, 8, 7, 5], 2, 2), [11, 15, 15, 15, 12]);
    }

    #[test]
    fn test() {}
}

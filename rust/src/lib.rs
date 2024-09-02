mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn num_squares(n: i32) -> i32 {
    let candidates: Vec<_> = (1..).map(|v| v * v).take_while(|&v| v <= n).collect();
    if candidates.last().is_some_and(|&v| v == n) {
        1
    } else if two_sum(&candidates, n) {
        2
    } else if three_sum(&candidates, n) {
        3
    } else {
        4
    }
}

fn two_sum(nums: &[i32], n: i32) -> bool {
    let mut map = std::collections::HashSet::new();
    for &num in nums.iter() {
        let comp = n - num;
        if map.contains(&comp) || comp == num {
            return true;
        } else {
            map.insert(num);
        }
    }
    false
}

fn three_sum(nums: &[i32], n: i32) -> bool {
    nums.iter().any(|&num| two_sum(nums, n - num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_squares(12), 3);
        debug_assert_eq!(num_squares(13), 2);
    }

    #[test]
    fn test() {}
}

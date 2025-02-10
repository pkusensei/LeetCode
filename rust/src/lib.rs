mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_floored_pairs(nums: &[i32]) -> i32 {
    const MOD: usize = 1_000_000_007;
    let max = *nums.iter().max().unwrap() as usize;
    let count = nums.iter().fold(vec![0; 1 + max], |mut acc, &num| {
        acc[num as usize] += 1;
        acc
    });
    let prefix = count.iter().fold(vec![], |mut acc, &c| {
        acc.push(c + acc.last().unwrap_or(&0));
        acc
    });
    let mut res = 0;
    for num in 1..=max {
        if count[num] == 0 {
            continue;
        }
        for mul in (num..=max).step_by(num) {
            let low = mul;
            let high = max.min(mul + num - 1);
            let c = prefix[high] - prefix[low - 1];
            res += count[num] * c * (mul / num);
            res %= MOD;
        }
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
        assert_eq!(sum_of_floored_pairs(&[2, 5, 9]), 10);
        assert_eq!(sum_of_floored_pairs(&[7, 7, 7, 7, 7, 7, 7]), 49);
    }

    #[test]
    fn test() {}
}

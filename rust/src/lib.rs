mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sub_array_ranges(nums: &[i32]) -> i64 {
    let n = nums.len();
    let [mut mins, mut maxs] = [0, 1].map(|_| Vec::with_capacity(n));
    let [mut incst, mut decst] = [0, 1].map(|_| vec![]);
    for (idx, &num) in nums.iter().enumerate() {
        while incst.last().is_some_and(|&i| nums[i] >= num) {
            incst.pop();
        }
        if let Some(&i) = incst.last() {
            mins.push(mins[i] + (idx - i) as i64 * i64::from(num));
        } else {
            mins.push((1 + idx) as i64 * i64::from(num));
        }
        incst.push(idx);

        while decst.last().is_some_and(|&i| nums[i] <= num) {
            decst.pop();
        }
        if let Some(&i) = decst.last() {
            maxs.push(maxs[i] + (idx - i) as i64 * i64::from(num));
        } else {
            maxs.push((1 + idx) as i64 * i64::from(num));
        }
        decst.push(idx);
    }
    maxs.iter().sum::<i64>() - mins.iter().sum::<i64>()
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
        assert_eq!(sub_array_ranges(&[1, 2, 3]), 4);
        assert_eq!(sub_array_ranges(&[1, 3, 3]), 4);
        assert_eq!(sub_array_ranges(&[4, -2, -3, 4, 1]), 59);
    }

    #[test]
    fn test() {}
}

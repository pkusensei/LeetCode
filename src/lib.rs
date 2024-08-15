mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_gap(nums: &[i32]) -> i32 {
    let size = nums.len();
    let (low, high) = (
        nums.iter().min().copied().unwrap_or(0),
        nums.iter().max().copied().unwrap_or(0),
    );
    if size <= 2 || low == high {
        return high - low;
    }

    // let bucket_size = (high - low) / (size as i32 - 1);
    let mut buckets = vec![vec![]; size - 1];
    for &num in nums.iter() {
        let idx = if num == high {
            size - 2
        } else {
            ((num - low) * (size as i32 - 1) / (high - low)) as usize
        };
        buckets[idx].push(num);
    }
    let buckets: Vec<_> = buckets.into_iter().filter(|v| !v.is_empty()).collect();
    buckets
        .windows(2)
        .map(|w| w[1].iter().min().unwrap() - w[0].iter().max().unwrap())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(maximum_gap(&[3, 6, 9, 1]), 3);
        debug_assert_eq!(maximum_gap(&[10]), 0);
    }

    #[test]
    fn test() {}
}

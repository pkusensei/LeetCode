mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn h_index(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut count = vec![0; n + 1];
    for &num in nums {
        let num = num as usize;
        if num < n {
            count[num] += 1
        } else {
            count[n] += 1
        }
    }
    let mut suffix = vec![0; n + 1];
    for idx in (0..n + 1).rev() {
        suffix[idx] = count[idx] + *suffix.get(idx + 1).unwrap_or(&0);
        if suffix[idx] as usize >= idx {
            return idx as i32;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(h_index(&[3, 0, 6, 1, 5]), 3);
        debug_assert_eq!(h_index(&[1, 3, 1]), 1);
    }

    #[test]
    fn test() {}
}

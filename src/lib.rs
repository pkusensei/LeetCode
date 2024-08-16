mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn two_sum(numbers: &[i32], target: i32) -> Vec<i32> {
    let (mut i, mut j) = (0, numbers.len() - 1);
    while i < j {
        match (numbers[i] + numbers[j]).cmp(&target) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Equal => return vec![(i + 1) as i32, (j + 1) as i32],
            std::cmp::Ordering::Greater => j -= 1,
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(two_sum(&[2, 7, 11, 15], 9), [1, 2]);
        debug_assert_eq!(two_sum(&[2, 3, 4], 6), [1, 3]);
        debug_assert_eq!(two_sum(&[-1, 0], -1), [1, 2]);
    }

    #[test]
    fn test() {}
}

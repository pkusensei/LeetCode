pub fn candy(ratings: &[i32]) -> i32 {
    let size = ratings.len();
    let mut nums = vec![1; size];
    for i in 1..size {
        if ratings[i - 1] < ratings[i] {
            nums[i] = 1 + nums[i - 1];
        }
    }
    for i in (0..size - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            nums[i] = nums[i].max(1 + nums[i + 1])
        }
    }
    nums.into_iter().sum()
}

// fn is_palindrome(s: &str) -> bool {
//     if s.len() < 2 {
//         return true;
//     }
//     s.bytes()
//         .rev()
//         .zip(s.bytes().take(s.len() / 2 + 1))
//         .all(|(b1, b2)| b1 == b2)
// }

// type Coord = (usize, usize);

// fn neighbors((a, b): Coord) -> impl Iterator<Item = Coord> {
//     [
//         (a.saturating_sub(1), b),
//         (a + 1, b),
//         (a, b.saturating_sub(1)),
//         (a, b + 1),
//     ]
//     .into_iter()
//     .filter(move |&p| p != (a, b))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(candy(&[1, 0, 2]), 5);
        debug_assert_eq!(candy(&[1, 2, 2]), 4);
    }

    #[test]
    fn test() {}
}

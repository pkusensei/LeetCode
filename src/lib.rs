pub fn single_number(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, num| acc ^ num)
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
        debug_assert_eq!(single_number(&[2, 2, 1]), 1);
        debug_assert_eq!(single_number(&[4, 1, 2, 1, 2]), 4);
        debug_assert_eq!(single_number(&[1]), 1);
    }

    #[test]
    fn test() {}
}

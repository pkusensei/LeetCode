pub fn single_number(nums: &[i32]) -> i32 {
    fn solve(nums: &[i32]) -> i32 {
        let (mut ones, mut twos) = (0, 0);
        for &num in nums {
            ones ^= num & !twos;
            twos ^= num & !ones;
        }
        ones
    }

    let mut ans = 0;
    // count 1s on every bit
    // count%3 reveals the bit from the single
    for i in 0..32 {
        let sum = nums.iter().fold(0, |acc, &num| {
            let n = (num >> i) & 1;
            acc + n
        });
        ans |= (sum % 3) << i;
    }
    ans
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
        debug_assert_eq!(single_number(&[2, 2, 3, 2]), 3);
        debug_assert_eq!(single_number(&[0, 1, 0, 1, 0, 1, 99]), 99);
    }

    #[test]
    fn test() {}
}

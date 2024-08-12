pub fn can_complete_circuit(gas: &[i32], cost: &[i32]) -> i32 {
    if gas.iter().sum::<i32>() < cost.iter().sum::<i32>() {
        return -1;
    }

    gas.iter()
        .zip(cost)
        .enumerate()
        .fold((0, 0), |(sum, start), (idx, (&gas, &cost))| {
            let sum = sum + gas - cost;
            if sum < 0 {
                (0, idx + 1)
            } else {
                (sum, start)
            }
        })
        .1 as i32
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
        debug_assert_eq!(can_complete_circuit(&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2]), 3);
        debug_assert_eq!(can_complete_circuit(&[2, 3, 4], &[3, 4, 3]), -1);
    }

    #[test]
    fn test() {}
}

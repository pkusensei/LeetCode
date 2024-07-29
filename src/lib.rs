pub fn num_teams(rating: &[i32]) -> i32 {
    rating
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, rat)| {
            let left_inc = rating[..idx].iter().filter(|&&n| n < rat).count();
            let right_inc = rating[idx + 1..].iter().filter(|&&n| rat < n).count();
            let left_dec = rating[..idx].iter().filter(|&&n| n > rat).count();
            let right_dec = rating[idx + 1..].iter().filter(|&&n| rat > n).count();
            left_inc * right_inc + left_dec * right_dec
        })
        .sum::<usize>() as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_teams(&[2, 5, 3, 4, 1]), 3);
        debug_assert_eq!(num_teams(&[2, 1, 3]), 0);
        debug_assert_eq!(num_teams(&[1, 2, 3, 4]), 4);
    }

    #[test]
    fn test() {}
}

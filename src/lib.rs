pub fn my_sqrt(x: i32) -> i32 {
    match x {
        0 => 0,
        1..4 => 1,
        _ => {
            let (mut left, mut right) = (1 as u64, (x / 2) as u64);
            while left <= right {
                let mid = (right - left) / 2 + left;
                match (mid * mid).cmp(&(x as u64)) {
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Equal => return mid as _,
                    std::cmp::Ordering::Greater => right = mid - 1,
                }
            }
            right as _
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(my_sqrt(4), 2);
        debug_assert_eq!(my_sqrt(8), 2);
        debug_assert_eq!(my_sqrt(9), 3);
        debug_assert_eq!(my_sqrt(12), 3);
    }

    #[test]
    fn test() {
        debug_assert_eq!(my_sqrt(2147395599), 46339);
    }
}

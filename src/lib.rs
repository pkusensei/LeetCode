pub fn count_seniors(details: &[&str]) -> i32 {
    details
        .iter()
        .filter_map(|line| line[11..13].parse::<i32>().ok())
        .filter(|&n| n > 60)
        .count() as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            count_seniors(&["7868190130M7522", "5303914400F9211", "9273338290F4010"]),
            2
        );
        debug_assert_eq!(count_seniors(&["1313579440F2036", "2921522980M5644"]), 0)
    }

    #[test]
    fn test() {}
}

pub fn minimum_pushes(s: &str) -> i32 {
    let mut freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[(b - b'a') as usize] += 1;
        acc
    });
    freq.sort_unstable_by_key(|&count| std::cmp::Reverse(count));
    freq.chunks(8)
        .enumerate()
        .map(|(idx, w)| w.iter().sum::<i32>() * (idx as i32 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(minimum_pushes("abcde"), 5);
        debug_assert_eq!(minimum_pushes("xyzxyzxyzxyz"), 12);
        debug_assert_eq!(minimum_pushes("aabbccddeeffgghhiiiiii"), 24);
    }

    #[test]
    fn test() {}
}

mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn find_repeated_dna_sequences(s: &str) -> Vec<String> {
    if s.len() < 11 {
        return vec![];
    }
    let counts = (0..=s.len() - 10).fold(HashMap::new(), |mut acc, i| {
        *acc.entry(&s[i..i + 10]).or_insert(0) += 1;
        acc
    });
    counts
        .into_iter()
        .filter_map(|(s, c)| if c > 1 { Some(s.to_string()) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"),
            ["AAAAACCCCC", "CCCCCAAAAA"]
        );
        debug_assert_eq!(find_repeated_dna_sequences("AAAAAAAAAAAAA"), ["AAAAAAAAAA"]);
    }

    #[test]
    fn test() {}
}

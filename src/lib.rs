pub fn group_anagrams<'a>(strs: &[&'a str]) -> Vec<Vec<&'a str>> {
    use std::collections::HashMap;
    let res: HashMap<i128, Vec<_>> = strs.iter().fold(HashMap::new(), |mut acc, &s| {
        let hash = s.bytes().fold(1, |acc, b| acc * letter_to_prime(b));
        acc.entry(hash).or_default().push(s);
        acc
    });
    res.into_values().collect()
}

const fn letter_to_prime(byte: u8) -> i128 {
    match byte {
        b'a' => 2,
        b'b' => 3,
        b'c' => 5,
        b'd' => 7,
        b'e' => 11,
        b'f' => 13,
        b'g' => 17,
        b'h' => 19,
        b'i' => 23,
        b'j' => 29,
        b'k' => 31,
        b'l' => 37,
        b'm' => 41,
        b'n' => 43,
        b'o' => 47,
        b'p' => 53,
        b'q' => 59,
        b'r' => 61,
        b's' => 67,
        b't' => 71,
        b'u' => 73,
        b'v' => 79,
        b'w' => 83,
        b'x' => 89,
        b'y' => 97,
        b'z' => 101,
        _ => unreachable!(),
    }
}

#[allow(unused)]
fn f1<'a>(strs: &[&'a str]) -> Vec<Vec<&'a str>> {
    use std::collections::BTreeMap;
    fn count(s: &str) -> BTreeMap<u8, i32> {
        s.bytes().fold(BTreeMap::new(), |mut acc, b| {
            *acc.entry(b).or_insert(0) += 1;
            acc
        })
    }

    let res: BTreeMap<BTreeMap<u8, i32>, Vec<&str>> =
        strs.iter().fold(BTreeMap::new(), |mut acc, s| {
            let m = count(s);
            acc.entry(m).or_default().push(s);
            acc
        });

    res.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(
        //     group_anagrams(&["eat", "tea", "tan", "ate", "nat", "bat"]),
        //     [vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
        // );
        debug_assert_eq!(group_anagrams(&[""]), [[""]]);
        debug_assert_eq!(group_anagrams(&["a"]), [["a"]]);
    }

    #[test]
    fn test() {}
}

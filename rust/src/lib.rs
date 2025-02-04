mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_nice_substring(s: &str) -> String {
    dfs(s).to_string()
}

fn dfs(s: &str) -> &str {
    if s.len() < 2 {
        return "";
    }
    let [upper, lower] = s.bytes().fold([[false; 26]; 2], |[mut up, mut low], b| {
        if b.is_ascii_uppercase() {
            up[usize::from(b - b'A')] = true;
        } else {
            low[usize::from(b - b'a')] = true;
        }
        [up, low]
    });
    for (idx, b) in s.bytes().enumerate() {
        if upper[usize::from(b.to_ascii_uppercase() - b'A')]
            && lower[usize::from(b.to_ascii_lowercase() - b'a')]
        {
            continue;
        }
        let s1 = dfs(&s[..idx]);
        let s2 = dfs(&s[1 + idx..]);
        return if s1.len() >= s2.len() { s1 } else { s2 };
    }
    s
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(longest_nice_substring("YazaAay"), "aAa");
        assert_eq!(longest_nice_substring("bB"), "bB");
        assert_eq!(longest_nice_substring("c"), "");
    }

    #[test]
    fn test() {}
}

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
    let map = knowledge
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, v| {
            acc.insert(v[0].as_bytes(), v[1].as_bytes());
            acc
        });
    let mut res = Vec::with_capacity(s.len());
    let mut left = None;
    for (idx, b) in s.bytes().enumerate() {
        match b {
            b'(' => left = Some(idx),
            b')' => {
                let Some(prev) = left.take() else {
                    unreachable!()
                };
                res.extend_from_slice(
                    map.get(&s.as_bytes()[1 + prev..idx])
                        .unwrap_or(&"?".as_bytes()),
                );
            }
            _ if left.is_none() => res.push(b),
            _ => (),
        }
    }
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}

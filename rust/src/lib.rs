mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_rational_equal(s: String, t: String) -> bool {
    let Some((a, b)) = Rational::new(&s).zip(Rational::new(&t)) else {
        return false;
    };
    a == b
}

#[derive(PartialEq, Eq)]
struct Rational {
    int: i32,
    dec: Vec<u8>,
}

impl Rational {
    fn new(s: &str) -> Option<Rational> {
        if let Some((a, b)) = s.split_once('.') {
            let int = a.parse().ok()?;
            if b.is_empty() {
                return Some(Self {
                    int,
                    dec: vec![b'0'; 8],
                });
            }
            let mut it = b.split(['(', ')']);
            let mut dec = it.next().map(|v| v.as_bytes().to_vec())?;
            if let Some(repeat) = it.next() {
                if repeat.bytes().all(|b| b == b'9') {
                    if let Some(i) = dec.iter().rposition(|&b| b != b'9') {
                        dec[i] += 1;
                        dec[1 + i..].fill(b'0');
                        while dec.len() < 8 {
                            dec.push(b'0');
                        }
                        return Some(Self { int, dec });
                    } else {
                        return Some(Self {
                            int: 1 + int,
                            dec: vec![b'0'; 8],
                        });
                    }
                }
                for b in repeat.bytes().cycle() {
                    if dec.len() < 8 {
                        dec.push(b);
                    } else {
                        break;
                    }
                }
                Some(Self { int, dec })
            } else {
                while dec.len() < 8 {
                    dec.push(b'0');
                }
                Some(Self { int, dec })
            }
        } else {
            let int = s.parse().ok()?;
            Some(Self {
                int,
                dec: vec![b'0'; 8],
            })
        }
    }
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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

mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_rational_equal(s: &str, t: &str) -> bool {
    let (Ok(a), Ok(b)) = (s.parse::<Rat>(), t.parse::<Rat>()) else {
        return false;
    };
    a == b
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rat {
    int: i32,
    dec: Vec<u8>,
}

impl std::str::FromStr for Rat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((int, dec_str)) = s.split_once('.') {
            let int = int.parse().map_err(|_| ())?;
            if dec_str.is_empty() {
                return Ok(Self {
                    int,
                    dec: vec![b'0'; 8],
                });
            }
            let mut it = dec_str.split(&['(', ')']);
            let mut dec = it.next().map(|v| v.as_bytes().to_vec()).unwrap_or_default();
            if let Some(rpt) = it.next() {
                // (99)
                if rpt.bytes().all(|v| v == b'9') {
                    if let Some(idx) = dec.iter().rposition(|&v| v != b'9') {
                        dec[idx] += 1;
                        dec[1 + idx..].fill(b'0');
                        while dec.len() < 8 {
                            dec.push(b'0');
                        }
                        return Ok(Self { int, dec });
                    } else {
                        return Ok(Self {
                            int: 1 + int,
                            dec: vec![b'0'; 8],
                        });
                    }
                }
                for b in rpt.bytes().cycle() {
                    if dec.len() < 8 {
                        dec.push(b);
                        continue;
                    }
                    break;
                }
                Ok(Self { int, dec })
            } else {
                while dec.len() < 8 {
                    dec.push(b'0');
                }
                Ok(Self { int, dec })
            }
        } else {
            let int = s.parse().map_err(|_| ())?;
            Ok(Self {
                int,
                dec: vec![b'0'; 8],
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_rational_equal("0.(52)", "0.5(25)"));
        debug_assert!(is_rational_equal("0.1666(6)", "0.166(66)"));
        debug_assert!(is_rational_equal("0.9(9)", "1."));
    }

    #[test]
    fn test() {
        debug_assert!(is_rational_equal("1.0", "1"));
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}

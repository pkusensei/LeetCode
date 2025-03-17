mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    let [mut mcount, mut pcount, mut gcount] = [0; 3];
    let [mut m_right, mut p_right, mut g_right] = [0; 3];
    let prefix = travel.iter().fold(vec![0], |mut acc, &num| {
        acc.push(num + acc.last().unwrap());
        acc
    });
    for (idx, s) in garbage.iter().enumerate() {
        for b in s.bytes() {
            match b {
                b'M' => {
                    mcount += 1;
                    m_right = idx;
                }
                b'P' => {
                    pcount += 1;
                    p_right = idx;
                }
                _ => {
                    gcount += 1;
                    g_right = idx
                }
            }
        }
    }
    mcount + prefix[m_right] + pcount + prefix[p_right] + gcount + prefix[g_right]
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

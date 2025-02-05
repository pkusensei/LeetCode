mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn beauty_sum(s: &str) -> i32 {
    let mut res = 0;
    for i1 in 0..s.len() {
        let mut count = [0; 26];
        for b in s.bytes().skip(i1) {
            count[usize::from(b - b'a')] += 1;
            let max = *count.iter().max().unwrap_or(&0);
            let min = count.iter().filter(|&&v| v > 0).min();
            if let Some(&v) = min {
                res += max - v
            }
        }
    }
    res
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
    fn basics() {
        assert_eq!(beauty_sum("aabcb"), 5);
        assert_eq!(beauty_sum("aabcbaa"), 17);
    }

    #[test]
    fn test() {
        assert_eq!(beauty_sum("yanmbgztcccnknsdaaeiafnobjhxxnjrzbrldvcgwfdowfxhdoosxgfauwvdgxbjmqbtqafzdkzyyinuiqreawksafsepksdixauxzjozxyfxgbauaetlbagujhttzgouzhqplyc"),34401);
    }
}

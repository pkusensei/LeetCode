mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn score(cards: &[&str], x: char) -> i32 {
    let x = x as u8;
    let mut both = 0;
    let mut x_left = [0; 26];
    let mut x_right = [0; 26];
    for s in cards.iter().map(|s| s.as_bytes()) {
        if s == [x, x] {
            both += 1;
        } else if s[0] == x {
            x_left[usize::from(s[1] - b'a')] += 1;
        } else if s[1] == x {
            x_right[usize::from(s[0] - b'a')] += 1;
        }
    }
    let [p1, singles1] = solve(&x_left);
    let [p2, singles2] = solve(&x_right);
    let pairs = p1 + p2;
    let free = singles1 + singles2;
    let used = free.min(both);
    both -= used;
    let extra = pairs.min(both / 2);
    pairs + used + extra
}

fn solve(freq: &[i32]) -> [i32; 2] {
    let mut sum = 0;
    let mut max = 0;
    for &f in freq {
        sum += f;
        max = max.max(f);
    }
    let pairs = (sum - max).min(sum / 2); // if max is dorminant
    let singles = sum - 2 * pairs;
    [pairs, singles]
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
    fn basics() {
        assert_eq!(score(&["aa", "ab", "ba", "ac"], 'a'), 2);
        assert_eq!(score(&["aa", "ab", "ba"], 'a'), 1);
        assert_eq!(score(&["aa", "ab", "ba", "ac"], 'b'), 0);
    }

    #[test]
    fn test() {
        assert_eq!(
            score(
                &[
                    "ab", "aa", "ab", "bc", "cc", "bc", "bb", "ac", "bc", "bc", "aa", "aa", "ba",
                    "bc", "cb", "ba", "ac", "bb", "cb", "ac", "cb", "cb", "ba", "bc", "ca", "ba",
                    "bb", "cc", "cc", "ca", "ab", "bb", "bc", "ba", "ac", "bc", "ac", "ac", "bc",
                    "bb", "bc", "ac", "bc", "aa", "ba", "cc", "ac", "bb", "ba", "bb"
                ],
                'b'
            ),
            16
        );
        assert_eq!(score(&["ba", "aa", "ba", "ca"], 'a'), 2);
    }
}

mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn num_tile_possibilities(tiles: &str) -> i32 {
    let mut freq: Vec<_> = tiles
        .bytes()
        .fold([0; 26], |mut acc, b| {
            acc[usize::from(b - b'A')] += 1;
            acc
        })
        .into_iter()
        .filter(|&v| v > 0)
        .collect();
    // let n = tiles.len();
    // let mut set = HashSet::new();
    // backtrack(&mut freq, n, &mut vec![], &mut set);
    // set.len() as _
    find_seqs(&mut freq)
}

fn backtrack(freq: &mut [i32], n: usize, curr: &mut Vec<usize>, set: &mut HashSet<Vec<usize>>) {
    if n == 0 {
        if !curr.is_empty() {
            set.insert(curr.clone());
        }
        return;
    }
    backtrack(freq, n - 1, curr, set);
    let sz = freq.len();
    for i in 0..sz {
        if freq[i] == 0 {
            continue;
        }
        freq[i] -= 1;
        curr.push(i);
        backtrack(freq, n - 1, curr, set);
        curr.pop();
        freq[i] += 1
    }
}

fn find_seqs(freq: &mut [i32]) -> i32 {
    let mut res = 0;
    let sz = freq.len();
    for i in 0..sz {
        if freq[i] == 0 {
            continue;
        }
        res += 1;
        freq[i] -= 1;
        res += find_seqs(freq);
        freq[i] += 1;
    }
    res
}

pub fn with_math(tiles: &str) -> i32 {
    const fn fact(n: i32) -> i32 {
        if n <= 1 {
            1
        } else {
            n * fact(n - 1)
        }
    }

    fn count_perms(seq: &[u8]) -> i32 {
        let freq = seq.iter().fold([0; 26], |mut acc, b| {
            acc[usize::from(b - b'A')] += 1;
            acc
        });
        let mut total = fact(seq.len() as _);
        for v in freq {
            total /= fact(v);
        }
        total
    }

    fn generate(bytes: &[u8], curr: &mut Vec<u8>, seen: &mut HashSet<Vec<u8>>) -> i32 {
        match bytes {
            [] if seen.insert(curr.clone()) => count_perms(curr),
            [head, tail @ ..] => {
                let mut res = generate(tail, curr, seen);
                curr.push(*head);
                res += generate(tail, curr, seen);
                curr.pop();
                res
            }
            _ => 0,
        }
    }

    let mut bytes = tiles.as_bytes().to_vec();
    bytes.sort_unstable();
    generate(&bytes, &mut vec![], &mut HashSet::new()) - 1
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
        assert_eq!(num_tile_possibilities("AAB"), 8);
        assert_eq!(num_tile_possibilities("AAABBC"), 188);
        assert_eq!(num_tile_possibilities("V"), 1);

        assert_eq!(with_math("AAB"), 8);
        assert_eq!(with_math("AAABBC"), 188);
        assert_eq!(with_math("V"), 1);
    }

    #[test]
    fn test() {}
}

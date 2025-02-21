mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn score_of_students(s: &str, answers: &[i32]) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let correct = eval(s);
    let mut memo = vec![vec![HashSet::new(); 1 + n]; n];
    generate(s, 0, n, &mut memo);
    let others = &memo[0][n];
    answers
        .iter()
        .map(|v| {
            if *v == correct {
                5
            } else if others.contains(v) {
                2
            } else {
                0
            }
        })
        .sum()
}

fn generate(s: &[u8], left: usize, right: usize, memo: &mut [Vec<HashSet<i32>>]) {
    if left + 1 >= right {
        memo[left][right].insert(i32::from(s[left] - b'0'));
    }
    if !memo[left][right].is_empty() {
        return;
    }
    let mut res = HashSet::new();
    for mid in (1 + left..right).step_by(2) {
        let op = if s[mid] == b'+' {
            |a, b| a + b
        } else {
            |a, b| a * b
        };
        generate(s, left, mid, memo);
        generate(s, 1 + mid, right, memo);
        for &a in memo[left][mid].iter() {
            for &b in memo[1 + mid][right].iter() {
                let temp = op(a, b);
                if temp <= 1000 {
                    res.insert(temp);
                }
            }
        }
    }
    memo[left][right].extend(res);
}

fn eval(s: &[u8]) -> i32 {
    let mut stack = vec![];
    for &b in s.iter() {
        match b {
            b'*' => stack.push(-1),
            b if b.is_ascii_digit() && stack.last().is_some_and(|&v| v == -1) => {
                stack.pop(); // pop '*'
                let last = stack.last_mut().unwrap();
                *last *= i32::from(b - b'0');
            }
            b if b.is_ascii_digit() => stack.push(i32::from(b - b'0')),
            _ => (),
        }
    }
    stack.into_iter().sum()
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
        assert_eq!(score_of_students("7+3*1*2", &[20, 13, 42]), 7);
        assert_eq!(score_of_students("3+5*2", &[13, 0, 10, 13, 13, 16, 16]), 19);
        assert_eq!(score_of_students("6+0*1", &[12, 9, 6, 4, 8, 6]), 10);
    }

    #[test]
    fn test() {}
}

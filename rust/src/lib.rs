mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_number(pattern: &str) -> String {
    let n = pattern.len();
    let mut res = vec![b'9'; 1 + n];
    for b in b'1'..=b'9' {
        let mut used = [false; 9]; // could use a bitmask
        used[usize::from(b - b'1')] = true;
        backtrack(pattern.as_bytes(), &mut vec![b], &mut used, &mut res);
    }
    String::from_utf8(res).unwrap()
}

fn backtrack(pattern: &[u8], curr: &mut Vec<u8>, used: &mut [bool], res: &mut Vec<u8>) {
    match pattern {
        [] => {
            if curr.len() == res.len() && curr < res {
                *res = curr.clone()
            }
        }
        [head, tail @ ..] => {
            for b in b'1'..=b'9' {
                if used[usize::from(b - b'1')] {
                    continue;
                }
                if (*head == b'D' && curr.last().is_some_and(|&v| v > b))
                    || (*head == b'I' && curr.last().is_some_and(|&v| v < b))
                {
                    curr.push(b);
                    used[usize::from(b - b'1')] = true;
                    backtrack(tail, curr, used, res);
                    curr.pop();
                    used[usize::from(b - b'1')] = false;
                }
            }
        }
    }
}

pub fn with_stack(pattern: &str) -> String {
    let n = pattern.len();
    let mut res = Vec::with_capacity(n);
    let mut stack = vec![];
    for idx in 0..=n {
        stack.push(b'1' + idx as u8);
        if pattern.as_bytes().get(idx).is_none_or(|&v| v == b'I') {
            while let Some(v) = stack.pop() {
                res.push(v);
            }
        }
    }
    String::from_utf8(res).unwrap()
}

pub fn sliding_window(pattern: &str) -> String {
    let n = pattern.len();
    let mut res = vec![];
    let mut prev = 0;
    for curr in 0..=n {
        res.push(b'1' + curr as u8);
        if pattern.as_bytes().get(curr).is_none_or(|&v| v == b'I') {
            res[prev..].reverse();
            prev = 1 + curr;
        }
    }
    String::from_utf8(res).unwrap()
}

pub fn greedy(pattern: &str) -> String {
    let n = pattern.len();
    let mut res = vec![];
    let mut suffix_d = vec![0; 1 + n];
    for (idx, b) in pattern.bytes().enumerate().rev() {
        if b == b'D' {
            suffix_d[idx] = 1 + suffix_d[1 + idx];
        }
    }
    let [mut max_so_far, mut curr_max] = [0, 0];
    for idx in 0..=n {
        if pattern.as_bytes().get(idx).is_some_and(|&v| v == b'I') {
            max_so_far += 1;
            res.push(b'0' + max_so_far);
            max_so_far = max_so_far.max(curr_max);
            curr_max = 0;
        } else {
            let temp = 1 + max_so_far + suffix_d[idx];
            res.push(b'0' + temp);
            curr_max = curr_max.max(temp);
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
    fn basics() {
        assert_eq!(smallest_number("IIIDIDDD"), "123549876");
        assert_eq!(smallest_number("DDD"), "4321");

        assert_eq!(with_stack("IIIDIDDD"), "123549876");
        assert_eq!(with_stack("DDD"), "4321");

        assert_eq!(sliding_window("IIIDIDDD"), "123549876");
        assert_eq!(sliding_window("DDD"), "4321");

        assert_eq!(greedy("IIIDIDDD"), "123549876");
        assert_eq!(greedy("DDD"), "4321");
    }

    #[test]
    fn test() {}
}

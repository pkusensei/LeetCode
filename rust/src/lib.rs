mod helper;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn remove_invalid_parentheses(s: &str) -> Vec<String> {
    bfs(s.to_string())
    // let (left, right) = s.bytes().fold((0, 0), |(left, right), byte| match byte {
    //     b'(' => (left + 1, right),
    //     b')' => {
    //         let right = if left == 0 { right + 1 } else { right };
    //         let left = if left > 0 { left - 1 } else { left };
    //         (left, right)
    //     }
    //     _ => (left, right),
    // });
    // let mut res = HashSet::new();
    // dfs(s, 0, 0, left, right, &mut String::new(), &mut res);
    // res.into_iter().collect()
}

fn dfs(
    s: &str,
    left_count: i32,
    right_count: i32,
    left_rem: i32,
    right_rem: i32,
    expr: &mut String,
    res: &mut HashSet<String>,
) {
    if s.is_empty() {
        if left_rem == 0 && right_rem == 0 {
            res.insert(expr.clone());
        }
        return;
    }
    let byte = s.as_bytes()[0];
    if byte == b'(' && left_rem > 0 || byte == b')' && right_rem > 0 {
        dfs(
            &s[1..],
            left_count,
            right_count,
            left_rem - if byte == b'(' { 1 } else { 0 },
            right_rem - if byte == b')' { 1 } else { 0 },
            expr,
            res,
        )
    }
    expr.push(char::from(byte));
    if !b"()".contains(&byte) {
        dfs(
            &s[1..],
            left_count,
            right_count,
            left_rem,
            right_rem,
            expr,
            res,
        );
    } else if byte == b'(' {
        dfs(
            &s[1..],
            left_count + 1,
            right_count,
            left_rem,
            right_rem,
            expr,
            res,
        );
    } else if right_count < left_count {
        dfs(
            &s[1..],
            left_count,
            right_count + 1,
            left_rem,
            right_rem,
            expr,
            res,
        );
    }
    expr.pop();
}

fn bfs(s: String) -> Vec<String> {
    if is_valid(&s) {
        return vec![s];
    }

    let mut queue = VecDeque::from([s]);
    let mut res = HashSet::new();
    let mut found = false;
    while !queue.is_empty() && !found {
        let count = queue.len();
        let mut seen = HashSet::new();
        // All strings current in queue[..count] have the same amount of chars removed
        // They should be considered in one batch
        for _ in 0..count {
            if let Some(s) = queue.pop_front() {
                if is_valid(&s) {
                    res.insert(s.clone());
                    found = true;
                }
                if found {
                    // Only least removals wanted
                    continue;
                }
                for (idx, b) in s.bytes().enumerate() {
                    if !b"()".contains(&b) {
                        continue;
                    }
                    let curr = format!("{}{}", &s[..idx], &s[idx + 1..]);
                    if seen.insert(curr.clone()) {
                        queue.push_back(curr)
                    }
                }
            }
        }
    }
    res.into_iter().collect()
}

fn is_valid(chs: &str) -> bool {
    let mut count = 0;
    for byte in chs.bytes() {
        match byte {
            b'(' => count += 1,
            b')' if count == 0 => {
                return false;
            }
            b')' => count -= 1,
            _ => (),
        }
    }
    count == 0
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(remove_invalid_parentheses("()())()"), ["(())()", "()()()"]);
        sort_eq(
            remove_invalid_parentheses("(a)())()"),
            ["(a())()", "(a)()()"],
        );
        debug_assert_eq!(remove_invalid_parentheses(")("), [""]);
    }

    #[test]
    fn test() {
        sort_eq(remove_invalid_parentheses("(((k()(("), ["k()", "(k)"]);
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}

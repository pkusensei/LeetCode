mod dsu;
mod helper;
mod trie;

use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr},
};

#[allow(unused_imports)]
use helper::*;

pub fn min_operations_to_flip(expression: &str) -> i32 {
    let mut parens = HashMap::new();
    let mut stack = vec![];
    for (idx, b) in expression.bytes().enumerate() {
        if b == b'(' {
            stack.push(idx);
        } else if b == b')' {
            parens.insert(idx, stack.pop().unwrap());
        }
    }
    let n = expression.len();
    dfs(expression.as_bytes(), &parens, 0, n - 1)[1]
}

// [value of exp, cost to flip]
fn dfs(s: &[u8], parens: &HashMap<usize, usize>, left: usize, right: usize) -> [i32; 2] {
    if left == right {
        return [i32::from(s[left] - b'0'), 1];
    }
    let mid = *parens.get(&right).unwrap_or(&right); // default to single digit
    if left == mid {
        return dfs(s, parens, left + 1, right - 1); // discard ()
    }
    let [val1, cost1] = dfs(s, parens, left, mid - 2); // exp before operator
    let [val2, cost2] = dfs(s, parens, mid, right);
    let func: fn(i32, i32) -> i32 = if s[mid - 1] == b'|' {
        i32::bitor
    } else {
        i32::bitand
    };
    let cost = if val1 + val2 == 1 {
        1
        // 1|0 and 0|1 and 1&0 and 0&1 => flip the op
    } else {
        // 0|0 and 1&1 => flip either side
        // 0&0 and 1|1 => flip either side + flip the op
        cost1.min(cost2) + (val1 ^ i32::from(s[mid - 1] == b'&'))
    };
    [func(val1, val2), cost]
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
        assert_eq!(min_operations_to_flip("1&(0|1)"), 1);
        assert_eq!(min_operations_to_flip("(0&0)&(0&0&0)"), 3);
        assert_eq!(min_operations_to_flip("(0|(1|0&1))"), 1);
    }

    #[test]
    fn test() {}
}

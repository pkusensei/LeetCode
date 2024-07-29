pub fn longest_valid_parentheses(s: &str) -> i32 {
    // ")()())" => [0, 1, 1, 1, 1]
    let mut bits: Vec<bool> = (0..s.len()).map(|_| false).collect();
    let mut stack = vec![];
    for (idx, ch) in s.bytes().enumerate() {
        if stack.is_empty() || ch == b'(' {
            stack.push((idx, ch));
        } else if ch == b')' {
            if let Some(&(i, b'(')) = stack.last() {
                stack.pop();
                bits[i] = true;
                bits[idx] = true;
            } else {
                stack.push((idx, ch))
            }
        }
    }
    let mut consecutive = 0;
    let mut res = 0;
    for b in bits {
        if b {
            consecutive += 1;
            res = res.max(consecutive);
        } else {
            consecutive = 0
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(longest_valid_parentheses("(()"), 2);
        debug_assert_eq!(longest_valid_parentheses(")()())"), 4);
        debug_assert_eq!(longest_valid_parentheses(""), 0);
    }

    #[test]
    fn test() {}
}

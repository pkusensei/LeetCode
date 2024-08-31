mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn calculate(s: &str) -> i32 {
    let s = s.as_bytes();
    let mut curr = 0;
    let mut stack = vec![];
    let mut idx = 0;
    let mut last_op = b'+';
    while idx < s.len() {
        match s[idx] {
            b'+' | b'-' | b'*' | b'/' => {
                compute(last_op, &mut stack, curr);
                curr = 0;
                last_op = s[idx]
            }
            n if n.is_ascii_digit() => {
                let mut temp = i32::from(n - b'0');
                while idx + 1 < s.len() && s[idx + 1].is_ascii_digit() {
                    idx += 1;
                    temp = 10 * temp + i32::from(s[idx] - b'0');
                }
                curr = temp;
            }
            _ => (),
        }
        idx += 1;
    }
    compute(last_op, &mut stack, curr);
    stack.into_iter().sum()
}

fn compute(last_op: u8, stack: &mut Vec<i32>, curr: i32) {
    match last_op {
        b'+' => stack.push(curr),
        b'-' => stack.push(-curr),
        b'*' => {
            let top = stack.pop().unwrap();
            stack.push(top * curr);
        }
        b'/' => {
            let top = stack.pop().unwrap();
            stack.push(top / curr);
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(calculate("3+2*2"), 7);
        debug_assert_eq!(calculate(" 3/2 "), 1);
        debug_assert_eq!(calculate(" 3+5 / 2 "), 5);
    }

    #[test]
    fn test() {}
}

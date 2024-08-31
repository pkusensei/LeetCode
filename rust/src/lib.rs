mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn calculate(s: &str) -> i32 {
    let s = s.as_bytes();
    let mut stack = vec![];
    let mut curr = 0;
    let mut negative = false;
    let mut idx = 0;
    while idx < s.len() {
        match s[idx] {
            b'+' => (),
            b'-' => negative = !negative,
            b'(' => {
                stack.push((curr, negative));
                curr = 0;
                negative = false;
            }
            b')' => {
                let (prev, neg) = stack.pop().unwrap_or((0, false));
                curr = if neg { prev - curr } else { prev + curr }
            }
            n if n.is_ascii_digit() => {
                let mut temp = i32::from(n - b'0');
                while idx + 1 < s.len() && s[idx + 1].is_ascii_digit() {
                    idx += 1;
                    let digit = i32::from(s[idx] - b'0');
                    temp = temp * 10 + digit
                }
                if negative {
                    temp = -temp;
                    negative = !negative;
                }
                curr += temp;
            }
            _ => (),
        }
        idx += 1;
    }
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(calculate("1 + 1"), 2);
        debug_assert_eq!(calculate("2-1 + 2"), 3);
        debug_assert_eq!(calculate("(1+(4+5+2)-3)+(6+8)"), 23)
    }

    #[test]
    fn test() {}
}

mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn eval_rpn(tokens: &[&str]) -> i32 {
    fn arith_op(stack: &mut Vec<i32>, func: fn(i32, i32) -> i32) {
        let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
        stack.push(func(b, a))
    }

    let mut stack = vec![];
    for token in tokens {
        match *token {
            "+" => arith_op(&mut stack, |a, b| a + b),
            "-" => arith_op(&mut stack, |a, b| a - b),
            "*" => arith_op(&mut stack, |a, b| a * b),
            "/" => arith_op(&mut stack, |a, b| a / b),
            _ => {
                let num = token.parse::<i32>().unwrap_or(1);
                stack.push(num);
            }
        }
    }
    stack[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(eval_rpn(&["2", "1", "+", "3", "*"]), 9);
        debug_assert_eq!(eval_rpn(&["4", "13", "5", "/", "+"]), 6);
        debug_assert_eq!(
            eval_rpn(&["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]),
            22
        );
    }

    #[test]
    fn test() {}
}

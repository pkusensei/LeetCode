mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn evaluate(expression: &str) -> i32 {
    eval(expression, &HashMap::new())
}

fn eval(expr: &str, enclosing: &HashMap<&str, i32>) -> i32 {
    if let Ok(num) = expr.parse() {
        return num;
    }
    if !expr.starts_with('(') {
        return enclosing[&expr];
    }
    let mut scope = enclosing.clone();
    let Some((left, right)) = expr.split_once(' ') else {
        return 0;
    };
    let tokens = parse(&right[..right.len() - 1]);
    match left {
        "(add" => eval(tokens[0], &scope) + eval(tokens[1], &scope),
        "(mult" => eval(tokens[0], &scope) * eval(tokens[1], &scope),
        "(let" => {
            for chunk in tokens.chunks_exact(2) {
                let val = eval(chunk[1], &scope);
                scope.insert(chunk[0], val);
            }
            eval(tokens.last().unwrap(), &scope)
        }
        _ => unreachable!(),
    }
}

// Into stream of tokens, but keep (...) together
// e.g (let x 2 (mult x (let x 3 y 4 (add x y))))
// has been stripped to x 2 (mult x (let x 3 y 4 (add x y)))
// Then parse into ["x", "2", "(mult x (let x 3 y 4 (add x y)))"]
// This is done by tracking () pairs
fn parse(expr: &str) -> Vec<&str> {
    let mut res = vec![];
    let mut open = 0;
    let mut start = 0;
    for (idx, b) in expr.bytes().enumerate() {
        if b == b'(' {
            open += 1;
        }
        if b == b')' {
            open -= 1;
        }
        if open == 0 && b.is_ascii_whitespace() {
            res.push(&expr[start..idx]);
            start = idx + 1;
        }
    }
    if !expr[start..].is_empty() {
        res.push(&expr[start..]);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))"), 14);
        debug_assert_eq!(evaluate("(let x 3 x 2 x)"), 2);
        debug_assert_eq!(evaluate("(let x 1 y 2 x (add x y) (add x y))"), 5);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}

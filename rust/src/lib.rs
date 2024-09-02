mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn diff_ways_to_compute(expression: &str) -> Vec<i32> {
    solve(expression, &mut HashMap::new())
}

fn solve<'a>(expr: &'a str, seen: &mut HashMap<&'a str, Vec<i32>>) -> Vec<i32> {
    if let Some(v) = seen.get(expr) {
        return v.clone();
    }
    if let Ok(r) = expr.parse() {
        seen.insert(expr, vec![r]);
        return vec![r];
    }
    let mut res = vec![];
    for (idx, b) in expr
        .bytes()
        .enumerate()
        .filter(|(_, b)| b"+-*".contains(&b))
    {
        let left = solve(&expr[..idx], seen);
        let right = solve(&expr[idx + 1..], seen);
        for r1 in left {
            for r2 in &right {
                match b {
                    b'+' => res.push(r1 + r2),
                    b'-' => res.push(r1 - r2),
                    b'*' => res.push(r1 * r2),
                    _ => (),
                }
            }
        }
    }
    seen.insert(expr, res.clone());
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(diff_ways_to_compute(&"2-1-1"), [0, 2]);
        debug_assert_eq!(diff_ways_to_compute(&"2*3-4*5"), [-34, -14, -10, -10, 10]);
    }

    #[test]
    fn test() {}
}

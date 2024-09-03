mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn add_operators(num: &str, target: i32) -> Vec<String> {
    if num.is_empty() {
        return vec![];
    }
    let digits: Vec<_> = num.bytes().collect();
    let target = i64::from(target);

    solve(
        &digits,
        (0, 0),
        (0, target),
        &mut Vec::with_capacity(2 * digits.len()),
    )
}

fn solve(
    digits: &[u8],
    (prev, curr): (i64, i64),
    (value, target): (i64, i64),
    path: &mut Vec<String>,
) -> Vec<String> {
    if digits.is_empty() {
        if value == target && curr == 0 {
            return vec![path[1..].join("")]; // [1..] to exclude the initial '+'
        }
        return vec![];
    }
    let mut res = vec![];
    let curr = curr * 10 + i64::from(digits[0] - b'0');

    // no op
    // check to avoid leading zeros
    if curr > 0 {
        res.extend(solve(&digits[1..], (prev, curr), (value, target), path))
    }

    // add
    path.push("+".to_string());
    path.push(curr.to_string());
    res.extend(solve(&digits[1..], (curr, 0), (value + curr, target), path));
    path.pop();
    path.pop();

    if !path.is_empty() {
        // sub
        path.push("-".to_string());
        path.push(curr.to_string());
        res.extend(solve(
            &digits[1..],
            (-curr, 0),
            (value - curr, target),
            path,
        ));
        path.pop();
        path.pop();

        // mul
        path.push("*".to_string());
        path.push(curr.to_string());
        res.extend(solve(
            &digits[1..],
            (prev * curr, 0),
            (value - prev + prev * curr, target),
            path,
        ));
        path.pop();
        path.pop();
    }

    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(add_operators("123", 6), ["1*2*3", "1+2+3"]);
        sort_eq(add_operators("232", 8), ["2*3+2", "2+3*2"]);
        debug_assert!(add_operators("3456237490", 9191).is_empty());
    }

    #[test]
    fn test() {}

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

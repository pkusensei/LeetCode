mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn solve_equation(equation: &str) -> String {
    let Some(((ax, an), (bx, bn))) = equation.split_once('=').map(|(a, b)| (parse(a), parse(b)))
    else {
        return "No solution".to_string();
    };
    match (ax - bx, bn - an) {
        (0, 0) => "Infinite solutions".to_string(),
        (0, _) => "No solution".to_string(),
        _ => format!("x={}", (bn - an) / (ax - bx)),
    }
}

fn parse(s: &str) -> (i32, i32) {
    let (s, n) = (s.as_bytes(), s.len());
    let mut idx = 0;
    let mut sign = 1;
    let mut factor = 0;
    let mut num = 0;
    while idx < n {
        match s[idx] {
            b'+' => idx += 1,
            b'-' => {
                sign = -1;
                idx += 1
            }
            b'x' => {
                factor += sign;
                sign = 1;
                idx += 1;
            }
            _ => {
                let mut temp = 0;
                while idx < n && s[idx].is_ascii_digit() {
                    temp = temp * 10 + i32::from(s[idx] - b'0');
                    idx += 1;
                }
                if s.get(idx).is_some_and(|&b| b == b'x') {
                    factor += sign * temp;
                    idx += 1;
                } else {
                    num += sign * temp;
                }
                sign = 1;
            }
        }
    }
    (factor, num)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(solve_equation("x+5-3+x=6+x-2"), "x=2");
        debug_assert_eq!(solve_equation("x=x"), "Infinite solutions");
        debug_assert_eq!(solve_equation("2x=x"), "x=0");
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

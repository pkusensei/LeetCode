mod helper;
mod trie;

use std::{collections::HashMap, fmt::Display, ops::Mul};

#[allow(unused_imports)]
use helper::*;

pub fn basic_calculator_iv(expression: &str, evalvars: &[&str], evalints: &[i32]) -> Vec<String> {
    let map: HashMap<&str, i32> = evalvars
        .iter()
        .zip(evalints.iter())
        .map(|(s, n)| (*s, *n))
        .collect();
    let (mut est, mut pst) = (vec![], vec![]);
    let zero = Expr::with_param(0);
    let mut priority = 0;
    est.push(zero);
    pst.push(priority);
    let (s, n) = (expression.as_bytes(), expression.len());
    let mut idx = 0;
    while idx < n {
        let byte = s[idx];
        match byte {
            b'(' => priority += 2,
            b')' => priority -= 2,
            b'+' | b'-' | b'*' => {
                let mut curr = priority;
                if byte == b'*' {
                    curr += 1;
                }
                while pst.last().is_some_and(|&v| v >= curr) {
                    let (Some(e1), Some(e2)) = (est.pop(), est.pop()) else {
                        break;
                    };
                    pst.pop();
                    est.push(e2.calc(&e1));
                }
                let Some(v) = est.last_mut() else {
                    break;
                };
                v.op = byte;
                pst.push(curr);
            }
            b if b.is_ascii_digit() => {
                let mut num = 0;
                while s.get(idx).is_some_and(|v| v.is_ascii_digit()) {
                    num = 10 * num + i32::from(s[idx] - b'0');
                    idx += 1;
                }
                est.push(Expr::with_param(num));
                continue;
            }
            b if b.is_ascii_lowercase() => {
                let mut curr = idx;
                while s.get(curr).is_some_and(|v| v.is_ascii_lowercase()) {
                    curr += 1;
                }
                est.push(Expr::with_var(&expression[idx..curr], &map));
                idx = curr;
                continue;
            }
            _ => (),
        }
        idx += 1
    }
    while est.len() > 1 {
        let (Some(e1), Some(e2)) = (est.pop(), est.pop()) else {
            break;
        };
        est.push(e2.calc(&e1));
    }
    est.last().map(|e| e.to_strs()).unwrap_or_default()
}

fn combine<'a>(mut terms: Vec<Term<'a>>) -> Vec<Term<'a>> {
    terms.sort_unstable();
    let mut res: Vec<Term<'a>> = vec![];
    for t in terms.into_iter() {
        if let Some(v) = res.last_mut() {
            if v == &t {
                v.param += t.param;
                continue;
            }
        }
        res.push(t);
    }
    res
}

#[derive(Debug, Clone)]
struct Term<'a> {
    param: i32,
    vars: Vec<&'a str>,
}

impl<'a> PartialEq for Term<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.vars == other.vars
    }
}

impl<'a> Eq for Term<'a> {}

impl<'a> PartialOrd for Term<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Term<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .vars
            .len()
            .cmp(&self.vars.len())
            .then(self.vars.cmp(&other.vars)) // WTF
    }
}

impl<'a, 'b> Mul<&'b Term<'a>> for &'b Term<'a> {
    type Output = Term<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        let param = self.param * rhs.param;
        let mut vars: Vec<&str> = self.vars.iter().chain(rhs.vars.iter()).copied().collect();
        vars.sort_unstable();
        Term { param, vars }
    }
}

impl<'a> Display for Term<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.param == 0 {
            write!(f, "")
        } else {
            let mut s = self.param.to_string();
            for v in self.vars.iter() {
                s.push('*');
                s.push_str(v);
            }
            write!(f, "{s}")
        }
    }
}

impl<'a> Term<'a> {
    fn with_param(param: i32) -> Self {
        Self {
            param,
            vars: vec![],
        }
    }

    fn with_var(var: &'a str, map: &HashMap<&str, i32>) -> Self {
        if let Some(&v) = map.get(var) {
            Self::with_param(v)
        } else {
            Self {
                param: 1,
                vars: vec![var],
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Expr<'a> {
    op: u8,
    terms: Vec<Term<'a>>,
}

impl<'a> Default for Expr<'a> {
    fn default() -> Self {
        Self {
            op: b'+',
            terms: vec![],
        }
    }
}

impl<'a, 'b> Mul<&'b Expr<'a>> for &'b Expr<'a> {
    type Output = Expr<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut terms = vec![];
        for t1 in self.terms.iter() {
            for t2 in rhs.terms.iter() {
                terms.push(t1 * t2);
            }
        }
        Expr {
            terms: combine(terms),
            ..Default::default()
        }
    }
}

impl<'a> Expr<'a> {
    fn with_param(param: i32) -> Self {
        Self {
            terms: vec![Term::with_param(param)],
            ..Default::default()
        }
    }

    fn with_var(var: &'a str, map: &HashMap<&str, i32>) -> Self {
        Self {
            terms: vec![Term::with_var(var, map)],
            ..Default::default()
        }
    }

    fn calc(&self, other: &Self) -> Self {
        if self.op == b'*' {
            self * other
        } else {
            let mut terms = self.terms.clone();
            for t in other.terms.iter() {
                let mut t = t.clone();
                t.param *= if self.op == b'-' { -1 } else { 1 };
                terms.push(t)
            }
            Self {
                terms: combine(terms),
                ..Default::default()
            }
        }
    }

    fn to_strs(&self) -> Vec<String> {
        self.terms
            .iter()
            .map(|t| t.to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            basic_calculator_iv("e + 8 - a + 5", &["e"], &[1]),
            ["-1*a", "14"]
        );
        debug_assert_eq!(
            basic_calculator_iv(
                "e - 8 + temperature - pressure",
                &["e", "temperature"],
                &[1, 12]
            ),
            ["-1*pressure", "5"]
        );
        debug_assert_eq!(
            basic_calculator_iv("(e + 8) * (e - 8)", &[], &[]),
            ["1*e*e", "-64"]
        );
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

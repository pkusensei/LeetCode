mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use itertools::{Itertools, chain, izip};
use std::{collections::HashMap, fmt::Display, ops::Mul};

pub fn basic_calculator_iv(
    expression: String,
    evalvars: Vec<String>,
    evalints: Vec<i32>,
) -> Vec<String> {
    let (s, n) = (expression.as_bytes(), expression.len());
    let map: HashMap<&str, i32> = izip!(evalvars.iter(), evalints.iter())
        .map(|(s, &v)| (s.as_str(), v))
        .collect();
    let mut expr_st = vec![];
    let mut prio_st = vec![];
    let zero = Expr::with_param(0);
    let mut priority = 0;
    expr_st.push(zero);
    prio_st.push(priority);
    let mut idx = 0;
    while idx < n {
        match s[idx] {
            b'(' => priority += 2,
            b')' => priority -= 2,
            b'+' | b'-' | b'*' => {
                let mut curr_prio = priority;
                if s[idx] == b'*' {
                    curr_prio += 1;
                }
                while prio_st.last().is_some_and(|&top| top >= curr_prio) {
                    let (Some(e1), Some(e2)) = (expr_st.pop(), expr_st.pop()) else {
                        break;
                    };
                    prio_st.pop();
                    expr_st.push(e2.eval(&e1));
                }
                let Some(top) = expr_st.last_mut() else {
                    break;
                };
                top.op = s[idx];
                prio_st.push(curr_prio);
            }
            b'0'..=b'9' => {
                let mut param = 0;
                while let Some(v) = s.get(idx)
                    && v.is_ascii_digit()
                {
                    param = 10 * param + i32::from(v - b'0');
                    idx += 1;
                }
                expr_st.push(Expr::with_param(param));
                continue;
            }
            b'a'..=b'z' => {
                let mut curr = idx;
                while s.get(curr).is_some_and(|v| v.is_ascii_alphabetic()) {
                    curr += 1;
                }
                let name = &expression[idx..curr];
                let e = map
                    .get(&name)
                    .map(|&param| Expr::with_param(param))
                    .unwrap_or_else(|| Expr::with_var(name));
                expr_st.push(e);
                idx = curr;
                continue;
            }
            _ => (),
        }
        idx += 1;
    }
    while expr_st.len() > 1 {
        let (Some(e1), Some(e2)) = (expr_st.pop(), expr_st.pop()) else {
            break;
        };
        expr_st.push(e2.eval(&e1));
    }
    expr_st[0]
        .terms
        .iter()
        .map(|t| t.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[derive(Clone)]
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
            .then(self.vars.cmp(&other.vars))
    }
}

impl<'a, 'b> Mul<&'b Term<'a>> for &'b Term<'a> {
    type Output = Term<'a>;

    fn mul(self, rhs: &'b Term<'a>) -> Self::Output {
        let param = self.param * rhs.param;
        let vars = chain!(self.vars.iter(), rhs.vars.iter())
            .copied()
            .sorted_unstable()
            .collect_vec();
        Term { param, vars }
    }
}

impl<'a> Display for Term<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.param == 0 {
            write!(f, "")
        } else {
            write!(f, "{}", self.param)?;
            for v in self.vars.iter() {
                write!(f, "*{}", v)?;
            }
            Ok(())
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

    fn with_var(var: &'a str) -> Self {
        Self {
            param: 1,
            vars: vec![var],
        }
    }
}

#[derive(Clone)]
struct Expr<'a> {
    op: u8,
    terms: Vec<Term<'a>>,
}

impl<'a> Default for Expr<'a> {
    fn default() -> Self {
        Self {
            op: b'+',
            terms: Default::default(),
        }
    }
}

impl<'a, 'b> Mul<&'b Expr<'a>> for &'b Expr<'a> {
    type Output = Expr<'a>;

    fn mul(self, rhs: &'b Expr<'a>) -> Self::Output {
        let mut terms = vec![];
        for t1 in &self.terms {
            for t2 in &rhs.terms {
                terms.push(t1 * t2);
            }
        }
        Expr::with_terms(terms)
    }
}

impl<'a> Expr<'a> {
    fn with_param(param: i32) -> Self {
        Self {
            terms: vec![Term::with_param(param)],
            ..Default::default()
        }
    }

    fn with_var(var: &'a str) -> Self {
        Self {
            terms: vec![Term::with_var(var)],
            ..Default::default()
        }
    }

    fn with_terms(mut terms: Vec<Term<'a>>) -> Self {
        terms.sort_unstable();
        let mut res: Vec<Term<'_>> = vec![];
        for t in terms {
            if let Some(last) = res.last_mut()
                && last == &t
            {
                last.param += t.param;
            } else {
                res.push(t);
            }
        }
        Self {
            terms: res,
            ..Default::default()
        }
    }

    fn eval(&self, other: &Self) -> Self {
        if self.op == b'*' {
            self * other
        } else {
            let mut terms = self.terms.clone();
            for t in &other.terms {
                let mut t = t.clone();
                if self.op == b'-' {
                    t.param *= -1;
                }
                terms.push(t);
            }
            Self::with_terms(terms)
        }
    }
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
    fn basics() {}

    #[test]
    fn test() {}
}

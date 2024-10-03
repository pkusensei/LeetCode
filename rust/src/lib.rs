mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_valid(code: &str) -> bool {
    let (s, n) = (code.as_bytes(), code.len());
    if s[0] != b'<' || !s.iter().last().is_some_and(|&b| b == b'>') {
        return false;
    }
    let mut idx = 0;
    let mut stack = vec![];
    let mut has_tag = false;
    while idx < n {
        let mut is_end = false;
        if stack.is_empty() && has_tag {
            return false;
        }
        if s[idx] == b'<' {
            idx = if !stack.is_empty() && s[idx + 1] == b'!' {
                let Some(close) = code[idx + 1..].find("]]>").map(|v| v + idx + 1) else {
                    return false;
                };
                if !is_valid_cdata(&code[idx + 2..close]) {
                    return false;
                }
                close
            } else {
                if s[idx + 1] == b'/' {
                    idx += 1;
                    is_end = true;
                }
                let Some(close) = code[idx + 1..].find('>').map(|v| v + idx + 1) else {
                    return false;
                };
                if !is_valid_tagname(&code[idx + 1..close], is_end, &mut stack, &mut has_tag) {
                    return false;
                }
                close
            };
        }
        idx += 1
    }
    stack.is_empty()
}

fn is_valid_tagname<'a>(
    s: &'a str,
    is_end: bool,
    stack: &mut Vec<&'a str>,
    has_tag: &mut bool,
) -> bool {
    if !(1..=9).contains(&s.len()) || !s.bytes().all(|b| b.is_ascii_uppercase()) {
        return false;
    }
    if is_end {
        if stack.last().is_some_and(|v| *v == s) {
            stack.pop();
        } else {
            return false;
        }
    } else {
        stack.push(s);
        *has_tag = true;
    }
    true
}

fn is_valid_cdata(s: &str) -> bool {
    s.find("[CDATA[").is_some_and(|v| v == 0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_valid(
            "<DIV>This is the first line <![CDATA[<div>]]></DIV>"
        ));
        debug_assert!(is_valid(
            "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>"
        ));
        debug_assert!(!is_valid("<A>  <B> </A>   </B>"));
    }

    #[test]
    fn test() {
        debug_assert!(!is_valid(
            "<TRUe><![CDATA[123123]]]]><![CDATA[>123123]]></TRUe>"
        ));
    }

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

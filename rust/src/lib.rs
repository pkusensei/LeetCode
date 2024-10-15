mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn remove_comments(source: &[&str]) -> Vec<String> {
    let mut res = vec![];
    let mut in_block: Option<String> = None;
    for line in source.iter() {
        let mut s = *line;
        while !s.is_empty() {
            if let Some(ref mut r) = in_block {
                let Some(end) = s.find(r#"*/"#) else {
                    break;
                };
                s = &s[2 + end..];
                let line = s.find(r#"//"#);
                let block = s.find(r#"/*"#);
                match (line, block) {
                    (Some(a), None) => {
                        r.push_str(&s[..a]);
                        res.push(in_block.take().unwrap_or_default());
                        break;
                    }
                    (Some(a), Some(b)) if a < b => {
                        r.push_str(&s[..a]);
                        res.push(in_block.take().unwrap_or_default());
                        break;
                    }
                    (None, None) => {
                        r.push_str(s);
                        res.push(in_block.take().unwrap_or_default());
                        break;
                    }
                    (_, Some(a)) => {
                        r.push_str(&s[..a]);
                        s = &s[2 + a..];
                    }
                }
            } else {
                let line = s.find(r#"//"#);
                let block = s.find(r#"/*"#);
                match (line, block) {
                    (Some(a), None) => {
                        res.push(s[..a].to_string());
                        break;
                    }
                    (Some(a), Some(b)) if a < b => {
                        res.push(s[..a].to_string());
                        break;
                    }
                    (None, None) => {
                        res.push(s.to_string());
                        break;
                    }
                    (_, Some(a)) => {
                        let r = s[..a].to_string();
                        in_block = Some(r);
                        s = &s[2 + a..];
                    }
                }
            }
        }
    }
    res.into_iter().filter(|s| !s.is_empty()).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            remove_comments(&[
                "/*Test program */",
                "int main()",
                "{ ",
                "  // variable declaration ",
                "int a, b, c;",
                "/* This is a test",
                "   multiline  ",
                "   comment for ",
                "   testing */",
                "a = b + c;",
                "}"
            ]),
            ["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"]
        );
        debug_assert_eq!(
            remove_comments(&["a/*comment", "line", "more_comment*/b"]),
            ["ab"]
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

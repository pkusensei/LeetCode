pub fn is_number(s: &str) -> bool {
    let mut iter = s.trim().bytes().peekable();
    let mut prev = None;
    let mut seen_e = false;
    let mut seen_point = false;
    let mut seen_plus_minus = false;
    while let Some(byte) = iter.next() {
        match byte {
            b'0'..=b'9' => prev = Some(byte),
            b'+' | b'-' => {
                if prev.is_some_and(|b| b != b'e') || seen_plus_minus || iter.peek().is_none() {
                    return false;
                }
                prev = Some(byte);
                seen_plus_minus = true
            }
            b'E' | b'e' => match prev {
                None => return false,
                Some(b) => {
                    if !b.is_ascii_digit() || seen_e || iter.peek().is_none() {
                        return false;
                    }
                    prev = Some(byte.to_ascii_lowercase());
                    seen_e = true;
                    seen_plus_minus = false
                }
            },
            b'.' => {
                if seen_e || seen_point {
                    return false;
                }
                match prev {
                    None => {
                        if !iter
                            .peek()
                            .is_some_and(|&b| b.is_ascii_digit() || b == b'e')
                        {
                            return false;
                        }
                    }
                    Some(b) => {
                        if !b.is_ascii_digit() && b != b'+' && b != b'-' {
                            return false;
                        }
                        if (b == b'+' || b == b'-')
                            && !iter.peek().is_some_and(|p| p.is_ascii_digit())
                        {
                            return false;
                        }
                    }
                }
                seen_point = true;
                seen_plus_minus = false;
            }
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_number("0"));
        debug_assert!(!is_number("e"));
        debug_assert!(!is_number("."));
    }

    #[test]
    fn test() {
        debug_assert!(!is_number("+."))
    }
}

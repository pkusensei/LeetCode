pub fn restore_ip_addresses(s: &str) -> Vec<String> {
    parse(s, 4).into_iter().map(|v| v.join(".")).collect()
}

fn parse(s: &str, sec_count: usize) -> Vec<Vec<&str>> {
    if !(sec_count..=sec_count * 3).contains(&s.len()) {
        return vec![];
    }
    match sec_count {
        0 => unreachable!(),
        1 => {
            if s.len() > 1 && s.starts_with('0') {
                vec![]
            } else if s.parse::<u8>().is_ok() {
                vec![vec![s]]
            } else {
                vec![]
            }
        }
        _ => {
            let mut res = vec![];
            for i in 1..s.len() {
                let first = parse(&s[..i], 1);
                let rest = parse(&s[i..], sec_count - 1);
                if !first.is_empty() && !rest.is_empty() {
                    for v1 in &first {
                        for v2 in &rest {
                            let mut v = v1.clone();
                            v.extend(v2);
                            res.push(v);
                        }
                    }
                }
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(
        //     restore_ip_addresses("25525511135"),
        //     ["255.255.11.135", "255.255.111.35"]
        // );
        debug_assert_eq!(restore_ip_addresses("0000"), ["0.0.0.0"]);
        debug_assert_eq!(
            restore_ip_addresses("101023"),
            [
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ]
        );
    }

    #[test]
    fn test() {}
}

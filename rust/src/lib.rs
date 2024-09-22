mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn valid_ip_address(query_ip: &str) -> &str {
    let mut it = query_ip.split('.');
    if it.clone().count() == 4
        && it.all(|s| !(s.len() > 1 && s.starts_with('0')) && s.parse::<u8>().is_ok())
    {
        return "IPv4";
    }
    it = query_ip.split(':');
    if it.clone().count() == 8 && it.all(|s| s.len() < 5 && u16::from_str_radix(s, 16).is_ok()) {
        return "IPv6";
    }
    "Neither"
    // with_std_net(query_ip)
}

fn with_std_net(s: &str) -> &str {
    use std::net::{Ipv4Addr, Ipv6Addr};

    if s.contains("::") {
        return "Neither";
    }
    let v4 = s.parse::<Ipv4Addr>();
    let v6 = s.parse::<Ipv6Addr>();
    match (v4, v6) {
        (Ok(_), _) => "IPv4",
        (_, Ok(_)) => "IPv6",
        _ => "Neither",
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(valid_ip_address("172.16.254.1"), "IPv4");
        debug_assert_eq!(
            valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334"),
            "IPv6"
        );
        debug_assert_eq!(valid_ip_address("256.256.256.256"), "Neither");
    }

    #[test]
    fn test() {
        debug_assert_eq!(valid_ip_address("192.0.0.1"), "IPv4");
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

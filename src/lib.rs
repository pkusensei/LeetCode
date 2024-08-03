pub fn add_binary(a: &str, b: &str) -> String {
    let mut a: Vec<_> = a.bytes().rev().map(|b| b - b'0').collect();
    let mut b: Vec<_> = b.bytes().rev().map(|b| b - b'0').collect();
    let length = a.len().max(b.len());
    while a.len() < length {
        a.push(0);
    }
    while b.len() < length {
        b.push(0);
    }

    let mut res = Vec::with_capacity(length + 1);
    let mut carry = 0;
    for (n1, n2) in a.into_iter().zip(b) {
        let sum = n1 + n2 + carry;
        res.push(sum % 2);
        carry = sum / 2;
    }
    if carry > 0 {
        res.push(carry);
    }
    res.into_iter()
        .rev()
        .map(|b| char::from(b + b'0'))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(add_binary("11", "1"), "100");
        debug_assert_eq!(add_binary("1010", "1011"), "10101");
    }

    #[test]
    fn test() {}
}

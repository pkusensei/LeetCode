pub fn multiply(num1: &str, num2: &str) -> String {
    let num1: Vec<_> = num1.bytes().map(|b| b - b'0').collect();
    let num2: Vec<_> = num2.bytes().map(|b| b - b'0').collect();
    let mut digits: Vec<u8> = vec![0; num1.len() + num2.len()];

    for (i, &a) in num1.iter().rev().enumerate() {
        let mut carry = 0;
        for (j, &b) in num2.iter().rev().enumerate() {
            let sum = digits[i + j] + a * b + carry;
            digits[i + j] = sum % 10;
            carry = sum / 10;
        }
        if carry > 0 {
            digits[i + num2.len()] = carry
        }
    }
    while digits.last().is_some_and(|&b| b == 0) {
        digits.pop();
    }
    let res: String = digits
        .into_iter()
        .rev()
        .map(|b| char::from(b + b'0'))
        .collect();
    if res.is_empty() {
        // digits are all 0s and popped
        0.to_string()
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(multiply("2", "3"), "6");
        debug_assert_eq!(multiply("123", "456"), "56088")
    }

    #[test]
    fn test() {
        debug_assert_eq!(multiply("0", "0"), "0");
    }
}

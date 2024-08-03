pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    if digits.is_empty() {
        return vec![1];
    }

    digits.reverse();
        let mut carry = 1;
        for v in digits.iter_mut() {
            if carry == 0 {
                break;
            }
            let sum = *v + carry;
            carry = sum / 10;
            *v = sum % 10;
        }
    if carry > 0 {
        digits.push(carry)
    };
    digits.reverse();
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(plus_one(vec![1, 2, 3]), [1, 2, 4]);
        debug_assert_eq!(plus_one(vec![4, 3, 2, 1]), [4, 3, 2, 2]);
        debug_assert_eq!(plus_one(vec![9]), [1, 0]);
    }

    #[test]
    fn test() {}
}

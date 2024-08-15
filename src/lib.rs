mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn lemonade_change(bills: &[i32]) -> bool {
    let mut five = 0;
    let mut ten = 0;

    for &b in bills.iter() {
        match b {
            5 => five += 1,
            10 => {
                if five > 0 {
                    five -= 1;
                    ten += 1;
                } else {
                    return false;
                }
            }
            20 => {
                if ten > 0 && five > 0 {
                    ten -= 1;
                    five -= 1;
                } else if five >= 3 {
                    five -= 3;
                } else {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(lemonade_change(&[5, 5, 5, 10, 20]));
        debug_assert!(!lemonade_change(&[5, 5, 10, 10, 20]));
    }

    #[test]
    fn test() {
        debug_assert!(lemonade_change(&[5, 5, 5, 20]));
    }
}

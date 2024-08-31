mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn majority_element(nums: &[i32]) -> Vec<i32> {
    //Boyer-Moore Voting Algorithm
    let (mut c1, mut n1, mut c2, mut n2) = (0, 0, 0, 0);
    for &num in nums {
        if num == n1 {
            c1 += 1
        } else if num == n2 {
            c2 += 1
        } else if c1 == 0 {
            n1 = num;
            c1 += 1
        } else if c2 == 0 {
            n2 = num;
            c2 += 1
        } else {
            c1 -= 1;
            c2 -= 1
        }
    }
    let (mut c1, mut c2) = (0, 0);
    for &num in nums {
        if num == n1 {
            c1 += 1
        } else if num == n2 {
            c2 += 1;
        }
    }
    let mut res = vec![];
    if c1 > nums.len() / 3 {
        res.push(n1)
    }
    if c2 > nums.len() / 3 {
        res.push(n2)
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(majority_element(&[3, 2, 3]), [3]);
        debug_assert_eq!(majority_element(&[1]), [1]);
        debug_assert_eq!(majority_element(&[1, 2]), [1, 2]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(majority_element(&[0, 0, 0]), [0]);
    }
}

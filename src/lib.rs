mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn is_happy(n: i32) -> bool {
    let mut seen = std::collections::HashSet::new();
    let mut curr = n;
    while seen.insert(curr) {
        curr = next_num(curr);
    }
    curr == 1

    // let (mut slow, mut fast) = (next_num(n), next_num(next_num(n)));
    // while fast != 1 && slow != fast {
    //     slow = next_num(slow);
    //     fast = next_num(next_num(fast));
    // }
    // fast == 1
}

fn next_num(n: i32) -> i32 {
    if n < 10 {
        n * n
    } else {
        (n % 10).pow(2) + next_num(n / 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_happy(19));
        debug_assert!(!is_happy(2));
    }

    #[test]
    fn test() {}
}

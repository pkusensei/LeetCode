mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn count_primes(n: i32) -> i32 {
    if n <= 2 {
        return 0;
    }
    let mut marks = vec![false; n as usize - 2];
    let offset = 2;
    let mut step = 2;
    while let Some(&mark) = marks.get(step - offset) {
        if !mark {
            let mut idx = step - offset + step;
            while let Some(m) = marks.get_mut(idx) {
                *m = true;
                idx += step;
            }
        }
        step += 1;
    }
    marks.into_iter().filter(|m| !m).count() as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(count_primes(10), 4);
        debug_assert_eq!(count_primes(0), 0);
        debug_assert_eq!(count_primes(1), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(count_primes(2), 0);
        debug_assert_eq!(count_primes(11), 4);
    }
}

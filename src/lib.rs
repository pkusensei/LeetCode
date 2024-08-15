mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn compare_version(version1: &str, version2: &str) -> i32 {
    let (mut it1, mut it2) = (version1.split('.'), version2.split('.'));
    loop {
        match (it1.next(), it2.next()) {
            (None, None) => break,
            (Some(s1), Some(s2)) => {
                let n1 = s1.parse::<u32>().unwrap();
                let n2 = s2.parse::<u32>().unwrap();
                if n1 > n2 {
                    return 1;
                }
                if n1 < n2 {
                    return -1;
                }
            }
            (Some(s1), None) => {
                let n1 = s1.parse::<u32>().unwrap();
                if n1 > 0 {
                    return 1;
                }
            }
            (None, Some(s2)) => {
                let n2 = s2.parse::<u32>().unwrap();
                if n2 > 0 {
                    return -1;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(compare_version("1.2", "1.10"), -1);
        debug_assert_eq!(compare_version("1.01", "1.001"), 0);
        debug_assert_eq!(compare_version("1.0", "1.0.0.0"), 0);
    }

    #[test]
    fn test() {}
}

mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn length_longest_path(input: &str) -> i32 {
    let mut stack = vec![];
    let mut res = 0;
    for line in input.split('\n') {
        let tab_count = line.bytes().take_while(|&b| b == b'\t').count();
        if stack.is_empty() {
            stack.push((tab_count, &line[tab_count..]));
        } else {
            while stack.last().is_some_and(|&(c, _)| c >= tab_count) {
                stack.pop();
            }
            stack.push((tab_count, &line[tab_count..]));
        }
        if line.contains('.') {
            let len = stack.len();
            res = res.max(stack.iter().map(|(_, s)| s.len()).sum::<usize>() + len - 1);
        }
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext"),
            20
        );
        debug_assert_eq!(length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"), 32);
        debug_assert_eq!(length_longest_path("a"), 0);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}

mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn summary_ranges(nums: &[i32]) -> Vec<String> {
    nums.chunk_by(|&a, &b| a + 1 == b)
        .map(|ch| {
            if ch.len() == 1 {
                format!("{}", ch[0])
            } else {
                format!("{}->{}", ch[0], *ch.last().unwrap())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(summary_ranges(&[0, 1, 2, 4, 5, 7]), ["0->2", "4->5", "7"]);
        debug_assert_eq!(
            summary_ranges(&[0, 2, 3, 4, 6, 8, 9]),
            ["0", "2->4", "6", "8->9"]
        );
    }

    #[test]
    fn test() {}
}

pub fn range_sum(nums: &[i32], _n: i32, left: i32, right: i32) -> i32 {
    const MODULUS: i32 = 10i32.pow(9) + 7;

    let mut queue: std::collections::BinaryHeap<_> = nums
        .iter()
        .copied()
        .enumerate()
        .map(|(i, num)| Pair { num, end: i })
        .collect();
    let mut res = 0;
    for i in 1..=right {
        // or 0..right; but later: left - 1 <= i
        let Some(Pair { num, end }) = queue.pop() else {
            return res;
        };
        if left <= i {
            res = (res + num) % MODULUS;
        }
        if let Some(&n) = nums.get(end + 1) {
            queue.push(Pair {
                num: num + n,
                end: end + 1,
            });
        }
    }
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair {
    num: i32,
    end: usize,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.num.cmp(&self.num).then(self.end.cmp(&other.end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(range_sum(&[1, 2, 3, 4], 4, 1, 2), 3);
        debug_assert_eq!(range_sum(&[1, 2, 3, 4], 4, 1, 5), 13);
        debug_assert_eq!(range_sum(&[1, 2, 3, 4], 4, 3, 4), 6);
        debug_assert_eq!(range_sum(&[1, 2, 3, 4], 4, 1, 10), 50);
    }

    #[test]
    fn test() {}
}

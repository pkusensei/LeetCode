mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct ExamTracker {
    data: Vec<(i32, i64)>,
}

impl ExamTracker {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn record(&mut self, time: i32, score: i32) {
        self.data.push((time, i64::from(score)));
    }

    fn total_score(&self, start_time: i32, end_time: i32) -> i64 {
        let a = self.data.partition_point(|v| v.0 < start_time);
        let b = self.data.partition_point(|v| v.0 <= end_time);
        self.data[a..b].iter().map(|v| v.1).sum()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        let mut examTracker = ExamTracker::new();
        examTracker.record(1, 98); // Alice takes a new exam at time 1, scoring 98.
        assert_eq!(examTracker.total_score(1, 1), 98); // Between time 1 and time 1, Alice took 1 exam at time 1, scoring 98. The total score is 98.
        examTracker.record(5, 99); // Alice takes a new exam at time 5, scoring 99.
        assert_eq!(examTracker.total_score(1, 3), 98); // Between time 1 and time 3, Alice took 1 exam at time 1, scoring 98. The total score is 98.
        assert_eq!(examTracker.total_score(1, 5), 197); // Between time 1 and time 5, Alice took 2 exams at time 1 and 5, scoring 98 and 99. The total score is 98 + 99 = 197.
        assert_eq!(examTracker.total_score(3, 4), 0); // Alice did not take any exam between time 3 and time 4. Therefore, the answer is 0.
        assert_eq!(examTracker.total_score(2, 5), 99);
    }

    #[test]
    fn test() {}
}

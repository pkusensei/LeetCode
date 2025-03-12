mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::VecDeque;

#[derive(Default)]
struct TextEditor {
    left: VecDeque<u8>,
    right: VecDeque<u8>,
}

impl TextEditor {
    fn new() -> Self {
        Default::default()
    }

    fn add_text(&mut self, text: String) {
        self.left.extend(text.bytes());
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let mut res = 0;
        while res < k && self.left.pop_back().is_some() {
            res += 1;
        }
        res
    }

    fn cursor_left(&mut self, mut k: i32) -> String {
        while k > 0 && !self.left.is_empty() {
            let b = self.left.pop_back().unwrap();
            self.right.push_front(b);
            k -= 1;
        }
        let n = self.left.len();
        let res: Vec<_> = self.left.range(n.saturating_sub(10)..).copied().collect();
        String::from_utf8(res).unwrap()
    }

    fn cursor_right(&mut self, mut k: i32) -> String {
        while k > 0 && !self.right.is_empty() {
            let b = self.right.pop_front().unwrap();
            self.left.push_back(b);
            k -= 1;
        }
        let n = self.left.len();
        let res: Vec<_> = self.left.range(n.saturating_sub(10)..).copied().collect();
        String::from_utf8(res).unwrap()
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
        let mut te = TextEditor::new(); // The current text is "|". (The '|' character represents the cursor)
        te.add_text("leetcode".into()); // The current text is "leetcode|".
        assert_eq!(te.delete_text(4), 4);
        // The current text is "leet|".
        // 4 characters were deleted.
        te.add_text("practice".into()); // The current text is "leetpractice|".
        assert_eq!(te.cursor_right(3), "etpractice");
        // The current text is "leetpractice|".
        // The cursor cannot be moved beyond the actual text and thus did not move.
        // "etpractice" is the last 10 characters to the left of the cursor.
        assert_eq!(te.cursor_left(8), "leet");
        // The current text is "leet|practice".
        // "leet" is the last min(10, 4) = 4 characters to the left of the cursor.
        assert_eq!(te.delete_text(10), 4);
        // The current text is "|practice".
        // Only 4 characters were deleted.
        assert_eq!(te.cursor_left(2), "");
        // The current text is "|practice".
        // The cursor cannot be moved beyond the actual text and thus did not move.
        // "" is the last min(10, 0) = 0 characters to the left of the cursor.
        assert_eq!(te.cursor_right(6), "practi");
        // The current text is "practi|ce".
        // "practi" is the last min(10, 6) = 6 characters to the left of the cursor.
    }

    #[test]
    fn test() {}
}

pub fn full_justify(words: &[&str], max_width: i32) -> Vec<String> {
    let lines = build_lines(words, max_width as _);
    let size = lines.len();
    lines
        .into_iter()
        .enumerate()
        .map(|(idx, line)| format_line(&line, max_width as _, idx == size - 1))
        .collect()
}

fn format_line(line: &[&str], max_width: usize, is_last: bool) -> String {
    let word_count = line.len();
    if word_count == 1 {
        let mut res = line[0].to_string();
        while res.len() < max_width {
            res.push(' ');
        }
        res
    } else if is_last {
        let mut res = String::new();
        for word in line {
            res.push_str(word);
            res.push(' ');
        }
        while res.len() > max_width {
            res.pop();
        }
        while res.len() < max_width {
            res.push(' ');
        }
        res
    } else {
        let word_len: usize = line.iter().map(|w| w.len()).sum();
        let space_count = word_count - 1;
        let space = max_width - word_len;
        let ave_width = space / space_count;
        let mut wide_count = space % space_count;
        let mut res = String::new();
        for word in line {
            res.push_str(word);
            res.extend(std::iter::repeat(' ').take(ave_width));
            if wide_count > 0 {
                res.push(' ');
                wide_count -= 1;
            }
        }
        res.trim_end().to_string()
    }
}

fn build_lines<T: AsRef<str>>(words: &[T], max_width: usize) -> Vec<Vec<&str>> {
    let mut res = vec![];
    let mut curr = vec![];
    for word in words {
        if curr.is_empty() {
            curr.push(word.as_ref())
        } else if line_length(&curr) + word.as_ref().len() < max_width {
            curr.push(word.as_ref());
        } else {
            res.push(curr);
            curr = vec![word.as_ref()];
        }
    }
    res.push(curr);
    res
}

fn line_length(line: &[&str]) -> usize {
    line.iter().map(|w| w.len()).sum::<usize>() + line.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            full_justify(
                &[
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ],
                16
            ),
            ["This    is    an", "example  of text", "justification.  "]
        );
        debug_assert_eq!(
            full_justify(&["What", "must", "be", "acknowledgment", "shall", "be"], 16),
            ["What   must   be", "acknowledgment  ", "shall be        "]
        );
        debug_assert_eq!(
            full_justify(
                &[
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do"
                ],
                20
            ),
            [
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ]
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            full_justify(
                &[
                    "ask", "not", "what", "your", "country", "can", "do", "for", "you", "ask",
                    "what", "you", "can", "do", "for", "your", "country"
                ],
                16
            ),
            [
                "ask   not   what",
                "your country can",
                "do  for  you ask",
                "what  you can do",
                "for your country"
            ]
        );
    }
}

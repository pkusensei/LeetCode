use std::collections::{HashMap, HashSet};

pub fn exist(board: &[&[char]], word: &str) -> bool {
    let board: HashMap<char, HashSet<_>> = board
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .copied()
                .enumerate()
                .map(move |(x, ch)| (ch, (x, y)))
        })
        .fold(HashMap::new(), |mut acc, (ch, pos)| {
            if word.contains(ch) {
                acc.entry(ch).or_default().insert(pos);
            }
            acc
        });
    let word: Vec<_> = word.chars().collect();
    let Some(v) = board.get(&word[0]) else {
        return false;
    };
    for &pos in v.iter() {
        if dfs(&board, &word[1..], &mut HashSet::from([pos]), pos) {
            return true;
        }
    }
    false
}

fn dfs(
    board: &HashMap<char, HashSet<(usize, usize)>>,
    word: &[char],
    seen: &mut HashSet<(usize, usize)>,
    prev: (usize, usize),
) -> bool {
    match word {
        [] => true,
        [head] => {
            let Some(v) = board.get(head) else {
                return false;
            };
            for curr in neighbors(prev).filter(|p| v.contains(p)) {
                if !seen.contains(&curr) {
                    return true;
                }
            }
            false
        }
        [head, ..] => {
            let Some(v) = board.get(head) else {
                return false;
            };
            for curr in neighbors(prev).filter(|p| v.contains(p)) {
                if seen.insert(curr) {
                    if dfs(board, &word[1..], seen, curr) {
                        return true;
                    } else {
                        seen.remove(&curr);
                    }
                }
            }
            false
        }
    }
}

fn neighbors((x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    [
        (x.saturating_sub(1), y),
        (x + 1, y),
        (x, y.saturating_sub(1)),
        (x, y + 1),
    ]
    .into_iter()
    .filter(move |&(nx, ny)| nx != x || ny != y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(exist(
            &[
                &['A', 'B', 'C', 'E'],
                &['S', 'F', 'C', 'S'],
                &['A', 'D', 'E', 'E']
            ],
            "ABCCED"
        ));
        debug_assert!(exist(
            &[
                &['A', 'B', 'C', 'E'],
                &['S', 'F', 'C', 'S'],
                &['A', 'D', 'E', 'E']
            ],
            "SEE"
        ));
        debug_assert!(!exist(
            &[
                &['A', 'B', 'C', 'E'],
                &['S', 'F', 'C', 'S'],
                &['A', 'D', 'E', 'E']
            ],
            "ABCB"
        ));
    }

    #[test]
    fn test() {
        debug_assert!(!exist(&[&['a']], "b"));
        debug_assert!(!exist(
            &[
                &['a', 'a', 'a', 'a'],
                &['a', 'a', 'a', 'a'],
                &['a', 'a', 'a', 'a']
            ],
            "aaaaaaaaaaaaa"
        ))
    }
}

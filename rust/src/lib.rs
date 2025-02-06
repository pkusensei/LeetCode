mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

struct AuthenticationManager {
    tokens: HashMap<String, i32>,
    limit: i32,
}

impl AuthenticationManager {
    fn new(limit: i32) -> Self {
        Self {
            tokens: HashMap::new(),
            limit,
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.tokens.insert(token_id, current_time);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        self.expire(current_time);
        if let Some(v) = self.tokens.get_mut(&token_id) {
            *v = current_time;
        }
    }

    fn expire(&mut self, current_time: i32) {
        let mut dels = vec![];
        for (k, v) in self.tokens.iter() {
            if v + self.limit < current_time {
                dels.push(k.clone());
            }
        }
        for k in dels {
            self.tokens.remove(&k);
        }
    }

    fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
        self.expire(current_time);
        self.tokens.len() as _
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
    fn basics() {}

    #[test]
    fn test() {}
}

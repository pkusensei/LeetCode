mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::{BTreeMap, HashMap, HashSet};
#[derive(Default)]
struct AuctionSystem {
    user_bid: HashMap<[i32; 2], i32>, // [user, item]-amount
    high: HashMap<i32, BTreeMap<i32, HashSet<i32>>>, // item-amount-user
}

impl AuctionSystem {
    fn new() -> Self {
        Default::default()
    }

    fn add_bid(&mut self, user_id: i32, item_id: i32, bid_amount: i32) {
        if self.user_bid.contains_key(&[user_id, item_id]) {
            self.update_bid(user_id, item_id, bid_amount);
        } else {
            self.user_bid.insert([user_id, item_id], bid_amount);
            self.high
                .entry(item_id)
                .or_default()
                .entry(bid_amount)
                .or_default()
                .insert(user_id);
        }
    }

    fn update_bid(&mut self, user_id: i32, item_id: i32, new_amount: i32) {
        let Some(old) = self.user_bid.insert([user_id, item_id], new_amount) else {
            unreachable!()
        };
        if let Some(map) = self.high.get_mut(&item_id)
            && let Some(user_set) = map.get_mut(&old)
        {
            user_set.remove(&user_id);
            if user_set.is_empty() {
                map.remove(&old);
            }
            map.entry(new_amount).or_default().insert(user_id);
        }
    }

    fn remove_bid(&mut self, user_id: i32, item_id: i32) {
        let Some(amount) = self.user_bid.remove(&[user_id, item_id]) else {
            unreachable!()
        };
        if let Some(map) = self.high.get_mut(&item_id)
            && let Some(user_set) = map.get_mut(&amount)
        {
            user_set.remove(&user_id);
            if user_set.is_empty() {
                map.remove(&amount);
            }
        }
    }

    fn get_highest_bidder(&self, item_id: i32) -> i32 {
        self.high
            .get(&item_id)
            .map(|map| {
                map.last_key_value()
                    .and_then(|set| set.1.iter().max().copied())
            })
            .flatten()
            .unwrap_or(-1)
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
    fn basics() {}

    #[test]
    fn test() {}
}

mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::{BinaryHeap, HashMap, hash_map::Entry};

#[derive(Debug, Default)]
struct AuctionSystem {
    user_bid: HashMap<[i32; 2], i32>, // [user, item] - amount
    // high: HashMap<i32, BTreeMap<i32, BTreeSet<i32>>>, // item-amount-user
    high: HashMap<i32, BinaryHeap<[i32; 2]>>, // item - [amount, user]
}

impl AuctionSystem {
    fn new() -> Self {
        Default::default()
    }

    fn add_bid(&mut self, user_id: i32, item_id: i32, bid_amount: i32) {
        if let Entry::Vacant(e) = self.user_bid.entry([user_id, item_id]) {
            e.insert(bid_amount);
            self.high
                .entry(item_id)
                .or_default()
                .push([bid_amount, user_id]);
        } else {
            self.update_bid(user_id, item_id, bid_amount);
        }
    }

    fn update_bid(&mut self, user_id: i32, item_id: i32, new_amount: i32) {
        let Some(old) = self.user_bid.insert([user_id, item_id], new_amount) else {
            unreachable!()
        };
        if let Some(heap) = self.high.get_mut(&item_id) {
            if let Some(&top) = heap.peek()
                && top == [old, user_id]
            {
                heap.pop();
            }
            heap.push([new_amount, user_id]);
        }
    }

    fn remove_bid(&mut self, user_id: i32, item_id: i32) {
        let Some(amount) = self.user_bid.remove(&[user_id, item_id]) else {
            unreachable!()
        };
        if let Some(heap) = self.high.get_mut(&item_id)
            && let Some(&top) = heap.peek()
            && top == [amount, user_id]
        {
            heap.pop();
        }
    }

    fn get_highest_bidder(&mut self, item_id: i32) -> i32 {
        if let Some(heap) = self.high.get_mut(&item_id) {
            while let Some(&[top_amount, user]) = heap.peek()
                && self
                    .user_bid
                    .get(&[user, item_id])
                    .is_none_or(|&curr| curr != top_amount)
            {
                heap.pop();
            }
            if let Some(top) = heap.peek() {
                return top[1];
            }
        }
        -1
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
        let mut auct = AuctionSystem::new(); // Initialize the Auction system
        auct.add_bid(1, 7, 5); // User 1 bids 5 on item 7
        auct.add_bid(2, 7, 6); // User 2 bids 6 on item 7
        assert_eq!(auct.get_highest_bidder(7), 2); // return 2 as User 2 has the highest bid
        auct.update_bid(1, 7, 8); // User 1 updates bid to 8 on item 7
        assert_eq!(auct.get_highest_bidder(7), 1); // return 1 as User 1 now has the highest bid
        auct.remove_bid(2, 7); // Remove User 2's bid on item 7
        assert_eq!(auct.get_highest_bidder(7), 1); // return 1 as User 1 is the current highest bidder
        assert_eq!(auct.get_highest_bidder(3), -1); // return -1 as no bids exist for item 3
    }

    #[test]
    fn test() {}
}

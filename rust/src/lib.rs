mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct MyLinkedList {
    head: Option<Box<Node>>,
}

impl MyLinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    fn get(&self, index: i32) -> i32 {
        let mut count = 0;
        let mut curr = self.head.as_ref();
        while curr.is_some() && count < index {
            count += 1;
            curr = curr.and_then(|n| n.next.as_ref());
        }
        curr.map(|n| n.val).unwrap_or(-1)
    }

    fn add_at_head(&mut self, val: i32) {
        let mut node = Node { val, next: None };
        node.next = self.head.take();
        self.head = Some(Box::new(node))
    }

    fn add_at_tail(&mut self, val: i32) {
        let node = Node { val, next: None };
        match self.head.as_mut() {
            None => self.head = Some(Box::new(node)),
            Some(mut curr) => {
                while let Some(ref mut next) = curr.next {
                    curr = next
                }
                curr.next = Some(Box::new(node));
            }
        }
    }

    fn add_at_index(&mut self, mut index: i32, val: i32) {
        if index <= 0 {
            self.add_at_head(val);
        }

        let mut curr = &mut self.head;
        let mut count = 0;
        while let Some(ref mut n) = curr {
            if count + 1 == index {
                let node = Node {
                    val,
                    next: n.next.take(),
                };
                n.next = Some(Box::new(node));
                return;
            }
            curr = &mut n.next;
            count += 1;
        }
    }

    fn delete_at_index(&mut self, mut index: i32) {
        let Some(ref mut head) = self.head else {
            return;
        };
        if index == 0 {
            let n = head.next.take();
            self.head = n;
            return;
        }
        let mut curr = &mut self.head;
        let mut count = 0;
        while let Some(ref mut n) = curr {
            if count + 1 == index {
                let Some(mut del) = n.next.take() else {
                    return;
                };
                n.next = del.next.take();
            }
            curr = &mut n.next;
            count += 1;
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut myLinkedList = MyLinkedList::new();
        myLinkedList.add_at_head(1);
        myLinkedList.add_at_tail(3);
        myLinkedList.add_at_index(1, 2); // linked list becomes 1->2->3
        debug_assert_eq!(myLinkedList.get(1), 2); // return 2
        myLinkedList.delete_at_index(1); // now the linked list is 1->3
        debug_assert_eq!(myLinkedList.get(1), 3); // return 3
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}

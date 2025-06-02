mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::{BTreeMap, HashMap};

struct TaskManager {
    t_up: HashMap<i32, [i32; 2]>,             // task-[user,prio]
    p_t_u: BTreeMap<i32, BTreeMap<i32, i32>>, // prio-task-user
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut t_up = HashMap::new();
        let mut p_t_u = BTreeMap::<_, BTreeMap<_, _>>::new();
        for task in tasks {
            let [uid, tid, prio] = task[..] else {
                unreachable!()
            };
            t_up.insert(tid, [uid, prio]);
            p_t_u.entry(prio).or_default().insert(tid, uid);
        }
        Self { t_up, p_t_u }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.t_up.insert(task_id, [user_id, priority]);
        self.p_t_u
            .entry(priority)
            .or_default()
            .insert(task_id, user_id);
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let Some(&[u_id, old_prio]) = self.t_up.get(&task_id) else {
            return;
        };
        let Some(old_p_map) = self.p_t_u.get_mut(&old_prio) else {
            return;
        };
        old_p_map.remove(&task_id);
        if old_p_map.is_empty() {
            self.p_t_u.remove(&old_prio);
        }
        self.t_up.insert(task_id, [u_id, new_priority]);
        self.p_t_u
            .entry(new_priority)
            .or_default()
            .insert(task_id, u_id);
    }

    fn rmv(&mut self, task_id: i32) {
        let Some([_uid, prio]) = self.t_up.remove(&task_id) else {
            return;
        };
        let Some(t_u) = self.p_t_u.get_mut(&prio) else {
            return;
        };
        t_u.remove(&task_id);
        if t_u.is_empty() {
            self.p_t_u.remove(&prio);
        }
    }

    fn exec_top(&mut self) -> i32 {
        let Some(t_u) = self.p_t_u.values_mut().next_back() else {
            return -1;
        };
        let Some((tid, uid)) = t_u.pop_last() else {
            return -1;
        };
        if t_u.is_empty() {
            self.p_t_u.pop_last();
        }
        self.t_up.remove(&tid);
        uid
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

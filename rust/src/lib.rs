mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

struct ThroneInheritance {
    ids: HashMap<String, usize>,
    children: HashMap<usize, Vec<usize>>,
    states: Vec<(String, bool)>,
}

impl ThroneInheritance {
    fn new(name: String) -> Self {
        Self {
            ids: HashMap::from([(name.clone(), 0)]),
            children: HashMap::new(),
            states: vec![(name, true)],
        }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        let cid = self.states.len();
        let pid = *self.ids.get(&parent_name).unwrap_or(&0);
        self.states.push((child_name.clone(), true));
        self.children.entry(pid).or_default().push(cid);
        self.ids.insert(child_name, cid);
    }

    fn death(&mut self, name: String) {
        let id = *self.ids.get(&name).unwrap_or(&0);
        self.states[id] = (name, false);
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        fn dfs(ti: &ThroneInheritance, id: usize, curr: &mut Vec<String>) {
            if ti.states[id].1 {
                curr.push(ti.states[id].0.to_owned());
            }
            let Some(children) = ti.children.get(&id) else {
                return;
            };
            for &c in children {
                dfs(ti, c, curr);
            }
        }

        let mut res = vec![];
        dfs(self, 0, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut t = ThroneInheritance::new("king".into()); // order: king
        t.birth("king".into(), "andy".into()); // order: king > andy
        t.birth("king".into(), "bob".into()); // order: king > andy > bob
        t.birth("king".into(), "catherine".into()); // order: king > andy > bob > catherine
        t.birth("andy".into(), "matthew".into()); // order: king > andy > matthew > bob > catherine
        t.birth("bob".into(), "alex".into()); // order: king > andy > matthew > bob > alex > catherine
        t.birth("bob".into(), "asha".into()); // order: king > andy > matthew > bob > alex > asha > catherine
        assert_eq!(
            t.get_inheritance_order(),
            [
                "king",
                "andy",
                "matthew",
                "bob",
                "alex",
                "asha",
                "catherine"
            ]
        ); // return ["king", "andy", "matthew", "bob", "alex", "asha", "catherine"]
        t.death("bob".into()); // order: king > andy > matthew > bob > alex > asha > catherine
        assert_eq!(
            t.get_inheritance_order(),
            ["king", "andy", "matthew", "alex", "asha", "catherine"]
        ); // return ["king", "andy", "matthew", "alex", "asha", "catherine"]
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}

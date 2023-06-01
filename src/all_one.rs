use std::{
    collections::{BTreeMap, HashMap, HashSet},
    rc::Rc,
};

type Addr = u16;
type Word = Rc<String>;
const NULL: Addr = 0;

#[derive(Default, Debug)]
pub struct AllOne {
    list: BTreeMap<Addr, HashSet<Word>>,
    word_map: HashMap<Word, Addr>,
}

impl AllOne {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn inc(&mut self, key: String) {
        let (word, new_addr) = if let Some(cur_addr) = self.word_map.remove(&key) {
            let new_addr = cur_addr + 1;
            let cur_node = self
                .list
                .get_mut(&cur_addr)
                .expect("word map should point existing nodes.");
            let word = cur_node
                .take(&key)
                .expect("word map should point to node which has the word.");
            if cur_node.is_empty() {
                self.list.remove(&cur_addr);
            }
            self.list.entry(new_addr).or_default().insert(word.clone());
            (word, new_addr)
        } else {
            let new_addr = 1;
            let word = Rc::new(key);
            self.list.entry(new_addr).or_default().insert(word.clone());
            (word, new_addr)
        };
        self.word_map.insert(word, new_addr);
    }

    pub fn dec(&mut self, key: String) {
        let cur_addr = self.word_map.remove(&key).expect("dec only occurs on key");
        let cur_node = self
            .list
            .get_mut(&cur_addr)
            .expect("word map should point existing nodes.");
        let word = cur_node
            .take(&key)
            .expect("word map should point to node with word");
        if cur_node.is_empty() {
            self.list.remove(&cur_addr);
        }
        if cur_addr - 1 != NULL {
            self.list
                .entry(cur_addr - 1)
                .or_default()
                .insert(word.clone());
            self.word_map.insert(word, cur_addr - 1);
        }
    }

    pub fn get_max_key(&mut self) -> String {
        self.list
            .iter()
            .rev()
            .next()
            .map(|(_, node)| {
                node.iter()
                    .next()
                    .expect("node cannot be empty")
                    .to_string()
            })
            .unwrap_or_default()
    }

    pub fn get_min_key(&mut self) -> String {
        self.list
            .iter()
            .next()
            .map(|(_, node)| {
                node.iter()
                    .next()
                    .expect("node cannot be empty")
                    .to_string()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut all_one = AllOne::new();
        all_one.inc("hello".into());
        all_one.inc("hello".into());
        assert_eq!(all_one.get_min_key(), "hello");
        assert_eq!(all_one.get_max_key(), "hello");
        all_one.inc("leet".into());
        assert_eq!(all_one.get_max_key(), "hello");
        assert_eq!(all_one.get_min_key(), "leet");
        all_one.dec("hello".into());
        all_one.inc("leet".into());

        assert_eq!(all_one.get_max_key(), "leet");
        assert_eq!(all_one.get_min_key(), "hello");

        all_one.dec("hello".into());
        all_one.dec("leet".into());
        all_one.dec("leet".into());

        assert_eq!(all_one.get_max_key(), "");
        assert_eq!(all_one.get_min_key(), "");
    }

    #[test]
    fn case2() {
        let mut all_one = AllOne::new();

        all_one.inc("a".into());

        for _ in 0..4 {
            all_one.inc("b".into());
        }

        for _ in 0..2 {
            all_one.dec("b".into());
        }

        assert_eq!(all_one.get_max_key(), "b");
        assert_eq!(all_one.get_min_key(), "a");
    }

    #[test]
    fn case3() {
        let mut all_one = AllOne::new();

        all_one.inc("a".into());
        dbg!("a inc 1 ", &all_one.list);

        for _ in 0..2 {
            all_one.inc("b".into());
        }
        dbg!("b inc 2", &all_one.list);

        for _ in 0..3 {
            all_one.inc("c".into());
        }
        dbg!("c inc 3", &all_one.list);

        for _ in 0..2 {
            all_one.dec("b".into());
            dbg!("b dec 2", &all_one.list);
        }

        assert_eq!(all_one.get_min_key(), "a");

        all_one.dec("a".into());
        dbg!("a dec 1", &all_one.list);

        assert_eq!(all_one.get_max_key(), "c");
        assert_eq!(all_one.get_min_key(), "c");
    }
    #[test]
    fn case4() {
        let mut all_one = AllOne::new();
        all_one.inc("a".into());
        all_one.inc("b".into());
        all_one.inc("c".into());
        all_one.inc("d".into());
        dbg!("abcd once", &all_one.list);
        all_one.inc("a".into());
        dbg!("abcd twice", &all_one.list);
        all_one.inc("b".into());
        dbg!("abcd twice", &all_one.list);
        all_one.inc("c".into());
        dbg!("abcd twice", &all_one.list);
        all_one.inc("d".into());
        dbg!("abcd twice", &all_one.list);

        all_one.inc("c".into());
        all_one.inc("d".into());
        all_one.inc("d".into());
        all_one.inc("a".into());
        dbg!("a dec 1", &all_one.list);

        assert_eq!(all_one.get_min_key(), "b");
    }

    #[test]
    fn case5() {
        let mut all_one = AllOne::new();
        // ["AllOne","inc","inc","inc","dec","inc","inc","getMaxKey"]
        //     [[],["hello"],["world"],["hello"],["world"],["hello"],["leet"],[]]

        all_one.inc("hello".into());
        all_one.inc("world".into());
        all_one.inc("hello".into());
        dbg!("before dec 1", &all_one.list);
        all_one.dec("world".into());
        dbg!("before dec 1", &all_one.list);
        all_one.inc("hello".into());
        dbg!("before dec 1", &all_one.list);
        all_one.inc("leet".into());
        assert_eq!(all_one.get_max_key(), "hello");
    }
}

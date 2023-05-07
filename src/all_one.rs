#![allow(dead_code)]
/// NOTE: btreemap is better but not available in leet rust
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    rc::Rc,
};

type Addr = u16;
type Word = Rc<String>;
const NULL: Addr = 0;

#[derive(Default, Debug)]
struct Node<T> {
    prev: Addr,
    next: Addr,
    value: T,
}

impl<T> Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[derive(Default, Debug)]
struct AllOne {
    head: Addr,
    tail: Addr,
    list: HashMap<Addr, Node<HashSet<Word>>>,
    word_map: HashMap<Word, Addr>,
}

impl AllOne {
    fn new() -> Self {
        Default::default()
    }

    fn insert_word(&mut self, at: Addr, next: Addr, prev: Addr, word: Word) {
        match self.list.entry(at) {
            std::collections::hash_map::Entry::Occupied(mut node) => {
                let node = node.get_mut();
                node.insert(word);
                if next != at {
                    node.next = next;
                }
                if prev != at {
                    node.prev = prev;
                }
            }
            std::collections::hash_map::Entry::Vacant(node) => {
                node.insert(Node {
                    prev,
                    next,
                    value: std::iter::once(word).collect(),
                });
                if next != NULL {
                    let mut next_node = self
                        .list
                        .get_mut(&next)
                        .expect("next node should be valid is not null");
                    next_node.prev = at;
                } else {
                    self.tail = at;
                }
                if prev != NULL {
                    let prev_node = self
                        .list
                        .get_mut(&prev)
                        .expect("previous node should be valid is not null");
                    prev_node.next = at;
                } else {
                    self.head = at;
                }
            }
        }
    }

    fn inc(&mut self, key: String) {
        let (word, new_addr) = if let Some(cur_addr) = self.word_map.get(&key) {
            let cur_addr = *cur_addr;
            let cur_node = self
                .list
                .get_mut(&cur_addr)
                .expect("word map should point existing nodes.");
            let word = cur_node
                .take(&key)
                .expect("word map should point to node which has the word.");
            let next = cur_node.next;
            let mut prev = cur_addr;
            if cur_node.is_empty() {
                prev = cur_node.prev;
                self.list.remove(&cur_addr).expect("should exists");
                if prev == NULL {
                    self.head = cur_addr + 1;
                }
            }
            self.insert_word(cur_addr + 1, next, prev, word.clone());
            (word, cur_addr + 1)
        } else {
            let word = Rc::new(key);
            // self.insert_word(1, self.head, NULL, word.clone());
            let node = self.list.entry(1).or_default();
            node.insert(word.clone());
            if self.head != 1 {
                node.next = self.head;
                self.head = 1;
            }
            (word, 1)
        };
        if self.tail < new_addr {
            self.tail = new_addr;
        }
        self.word_map.insert(word, new_addr);
    }

    fn dec(&mut self, key: String) {
        let cur_addr = self.word_map.remove(&key).expect("dec only occurs on key");
        let cur_node = self
            .list
            .get_mut(&cur_addr)
            .expect("word map should point existing nodes.");
        let word = cur_node
            .take(&key)
            .expect("word map should point to node with word");
        let mut next = cur_addr;
        let prev = cur_node.prev;
        if cur_node.is_empty() {
            next = cur_node.next;
            self.list.remove(&cur_addr).expect("should exists");
            if next == NULL {
                self.tail = cur_addr - 1;
            }
        }
        if cur_addr - 1 != NULL {
            self.insert_word(cur_addr - 1, next, prev, word.clone());
            self.word_map.insert(word, cur_addr - 1);
        } else {
            self.head = next;
            if prev == NULL && next != NULL {
                self.list
                    .get_mut(&next)
                    .expect("next node should not be null")
                    .prev = NULL
            }
        }
    }

    fn get_max_key(&mut self) -> String {
        self.list
            .get_mut(&self.tail)
            .map(|node| {
                node.iter()
                    .next()
                    .expect("node cannot be empty")
                    .to_string()
            })
            .unwrap_or_default()
    }

    fn get_min_key(&mut self) -> String {
        self.list
            .get_mut(&self.head)
            .map(|node| {
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
        dbg!("abcd once", &all_one.list, &all_one.head, &all_one.tail);
        all_one.inc("a".into());
        dbg!("abcd twice", &all_one.list, &all_one.head, &all_one.tail);
        all_one.inc("b".into());
        dbg!("abcd twice", &all_one.list, &all_one.head, &all_one.tail);
        all_one.inc("c".into());
        dbg!("abcd twice", &all_one.list, &all_one.head, &all_one.tail);
        all_one.inc("d".into());
        dbg!("abcd twice", &all_one.list, &all_one.head, &all_one.tail);

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

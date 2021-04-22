use std::collections::HashMap;
struct Trie {
    next: HashMap<char, Trie>,
    exist: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            next: HashMap::new(),
            exist: false,
        }
    }

    fn insert(&mut self, word: String) {
        let n = word.as_bytes().len();
        self.insert_char(0, &word, &n);
    }

    fn insert_char(&mut self, idx: usize, word: &String, n: &usize) {
        if idx == *n {
            self.exist = true;
            return;
        }
        let c: char = word.chars().clone().nth(idx).unwrap();
        match self.next.get_mut(&c) {
            Some(x) => {
                x.insert_char(idx + 1, word, n);
            }
            None => {
                let mut new_node = Trie::new();
                new_node.insert_char(idx + 1, word, n);
                self.next.insert(c, new_node);
            }
        }
    }

    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for c in word.chars() {
            if let Some(x) = cur.next.get(&c) {
                cur = x;
            } else {
                return false;
            }
        }

        cur.exist
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        for c in prefix.chars() {
            if let Some(x) = cur.next.get(&c) {
                cur = x;
            } else {
                return false;
            }
        }

        true
    }
}

fn main() {
    let mut obj = Trie::new();
    obj.insert(String::from("okay"));
    obj.insert(String::from("obey"));
    obj.insert(String::from("ko"));

    assert_eq!(obj.search(String::from("okay")), true);
    assert_eq!(obj.search(String::from("ko")), true);

    assert_eq!(obj.search(String::from("ob")), false);
    assert_eq!(obj.search(String::from("j")), false);
    assert_eq!(obj.search(String::from("")), false);

    assert_eq!(obj.starts_with(String::from("ob")), true);
    assert_eq!(obj.starts_with(String::from("ok")), true);
    assert_eq!(obj.starts_with(String::from("o")), true);
    assert_eq!(obj.starts_with(String::from("obe")), true);
    assert_eq!(obj.starts_with(String::from("k")), true);

    assert_eq!(obj.starts_with(String::from("j")), false);
    assert_eq!(obj.starts_with(String::from("a")), false);
}

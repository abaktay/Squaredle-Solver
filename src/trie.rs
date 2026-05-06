use std::fs::read_to_string;

#[derive(Default)]
pub struct TrieNode {
    pub children: [Option<Box<TrieNode>>; 26],
    pub is_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: Default::default(),
            is_word: false,
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            let idx = (c as u8 - b'a') as usize;
            node = node.children[idx].get_or_insert_default();
        }
        node.is_word = true;
    }

    pub fn load_file(&mut self, path: &str) {
        let list = read_to_string(path).unwrap();

        for word in list.lines() {
            let word = word.trim().to_lowercase();
            if word.len() >= 4 && word.chars().all(|c| c.is_ascii_alphabetic()) {
                self.insert(&word);
            }
        }
    }

    pub fn _collect_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        let mut current = String::new();
        Self::_collect_recursive(&self, &mut current, &mut words);
        words
    }

    fn _collect_recursive(node: &TrieNode, current: &mut String, words: &mut Vec<String>) {
        if node.is_word {
            words.push(current.clone());
        }
        for (i, child) in node.children.iter().enumerate() {
            if let Some(child_node) = child {
                current.push((b'a' + i as u8) as char);
                Self::_collect_recursive(child_node, current, words);
                current.pop();
            }
        }
    }
}

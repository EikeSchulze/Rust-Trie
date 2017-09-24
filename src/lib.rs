#![feature(entry_or_default)]

type Data = usize;
type Key = u8;

#[derive(Debug)]
pub struct TrieNode {
    data: Option<Data>,
    children: std::collections::HashMap<Key, TrieNode>
}

impl std::default::Default for TrieNode {
    fn default() -> Self {
        TrieNode {
            data: None,
            children: std::collections::HashMap::default()
        }
    }
}

impl TrieNode {
    pub fn new_empty() -> Self {
        TrieNode::default()
    }

    pub fn new_with_data(data: Data) -> Self {
        TrieNode {
            data: Some(data),
            children: std::collections::HashMap::default()
        }
    }

    pub fn put(&mut self, key_word: &[Key], data: Data) {
        if key_word.len() == 0 {
            self.data = Some(data);
        } else {
            self.children.entry(key_word[0]).or_default().put(&key_word[1..], data);
        }
    }

    pub fn get(&self, key_word: &[Key]) -> Option<Data> {
        if key_word.len() == 0 {
            self.data
        } else {
            self.children.get(&key_word[0]).and_then(|node| node.get(&key_word[1..]))
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    root_node: TrieNode
}

impl std::default::Default for Trie {
    fn default() -> Self {
        Trie {
            root_node: TrieNode::new_empty()
        }
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

    pub fn put(&mut self, key_word: &[Key], data: Data) {
        self.root_node.put(key_word, data);
    }

    pub fn get(&self, key_word: &[Key]) -> Option<Data> {
        self.root_node.get(key_word)
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_default() {
        let trie = Trie::default();
        let key_word: &[u8] = &[1, 2];
        assert_eq!(trie.get(key_word), None);
    }

    #[test]
    fn test_get_from_empty() {
        let trie = Trie::new();
        let key_word: &[u8] = &[1, 2, 3, 4];
        assert_eq!(trie.get(key_word), None);
    }

    #[test]
    fn test_put_and_get() {
        let mut trie = Trie::new();
        let key_word: &[u8] = &[1, 2, 3, 4];
        let data: usize = 42;
        assert_eq!(trie.get(key_word), None);
        trie.put(key_word, data);
        assert_eq!(trie.get(key_word), Some(data));
    }

    #[test]
    fn test_put_and_get_multiple() {
        let mut trie = Trie::new();
        let key_word_1: &[u8] = &[1];
        let key_word_2: &[u8] = &[1, 2];
        let key_word_3: &[u8] = &[1, 3];
        let data_1: usize = 42;
        let data_2: usize = 12;
        let data_3: usize = 56;
        assert_eq!(trie.get(key_word_1), None);
        assert_eq!(trie.get(key_word_2), None);
        assert_eq!(trie.get(key_word_3), None);
        trie.put(key_word_1, data_1);
        trie.put(key_word_2, data_2);
        trie.put(key_word_3, data_3);
        assert_eq!(trie.get(key_word_1), Some(data_1));
        assert_eq!(trie.get(key_word_2), Some(data_2));
        assert_eq!(trie.get(key_word_3), Some(data_3));
    }
}

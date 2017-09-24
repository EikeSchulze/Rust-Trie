#![feature(entry_or_default)]

#[derive(Debug)]
pub struct TrieNode<K: std::cmp::Eq + std::hash::Hash, V> {
    data: Option<V>,
    children: std::collections::HashMap<K, TrieNode<K, V>>
}

impl <K: std::cmp::Eq + std::hash::Hash, V> std::default::Default for TrieNode<K, V> {
    fn default() -> Self {
        TrieNode {
            data: None,
            children: std::collections::HashMap::default()
        }
    }
}

impl <K: std::cmp::Eq + std::hash::Hash + Copy, V> TrieNode<K, V> {
    pub fn new_empty() -> Self {
        TrieNode::default()
    }

    pub fn new_with_data(data: V) -> Self {
        TrieNode {
            data: Some(data),
            children: std::collections::HashMap::default()
        }
    }

    pub fn put(&mut self, key_word: &[K], data: V) {
        if key_word.len() == 0 {
            self.data = Some(data);
        } else {
            self.children.entry(key_word[0]).or_default().put(&key_word[1..], data);
        }
    }

    pub fn get(&self, key_word: &[K]) -> Option<&V> {
        if key_word.len() == 0 {
            self.data.as_ref()
        } else {
            self.children.get(&key_word[0]).and_then(|node| node.get(&key_word[1..]))
        }
    }
}

#[derive(Debug)]
pub struct Trie<K: std::cmp::Eq + std::hash::Hash, V> {
    root_node: TrieNode<K, V>
}

impl <K: std::cmp::Eq + std::hash::Hash + Copy, Data> std::default::Default for Trie<K, Data> {
    fn default() -> Self {
        Trie {
            root_node: TrieNode::new_empty()
        }
    }
}

impl <K: std::cmp::Eq + std::hash::Hash + Copy, Data> Trie<K, Data> {
    pub fn new() -> Self {
        Trie::default()
    }

    pub fn put(&mut self, key_word: &[K], data: Data) {
        self.root_node.put(key_word, data);
    }

    pub fn get(&self, key_word: &[K]) -> Option<&Data> {
        self.root_node.get(key_word)
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_default() {
        let trie: Trie<_, usize> = Trie::default();
        let key_word = [1, 2];
        assert_eq!(trie.get(&key_word), None);
    }

    #[test]
    fn test_get_from_empty() {
        let trie: Trie<_, usize> = Trie::new();
        let key_word = [1, 2, 3, 4];
        assert_eq!(trie.get(&key_word), None);
    }

    #[test]
    fn test_put_and_get() {
        let mut trie = Trie::new();
        let key_word = &[1, 2, 3, 4];
        let data = 42;
        assert_eq!(trie.get(key_word), None);
        trie.put(key_word, data);
        assert_eq!(trie.get(key_word), Some(&data));
    }

    #[test]
    fn test_put_and_get_multiple() {
        let mut trie = Trie::new();
        let key_word_1 = &[1];
        let key_word_2 = &[1, 2];
        let key_word_3 = &[1, 3];
        let data_1 = 42;
        let data_2 = 12;
        let data_3 = 56;
        assert_eq!(trie.get(key_word_1), None);
        assert_eq!(trie.get(key_word_2), None);
        assert_eq!(trie.get(key_word_3), None);
        trie.put(key_word_1, data_1);
        trie.put(key_word_2, data_2);
        trie.put(key_word_3, data_3);
        assert_eq!(trie.get(key_word_1), Some(&data_1));
        assert_eq!(trie.get(key_word_2), Some(&data_2));
        assert_eq!(trie.get(key_word_3), Some(&data_3));
    }

    #[test]
    fn test_str_keys() {
        let mut trie = Trie::new();
        let key_word_1 = "ABC";
        let key_word_2 = "AB";
        let key_word_3 = "A";
        let data_1 = "Steve";
        let data_2 = "Rembrandt";
        let data_3 = "Mike";
        trie.put(key_word_1.as_bytes(), data_1);
        trie.put(key_word_2.as_bytes(), data_2);
        trie.put(key_word_3.as_bytes(), data_3);
        assert_eq!(trie.get(key_word_1.as_bytes()), Some(&data_1));
        assert_eq!(trie.get(key_word_2.as_bytes()), Some(&data_2));
        assert_eq!(trie.get(key_word_3.as_bytes()), Some(&data_3));
    }
}

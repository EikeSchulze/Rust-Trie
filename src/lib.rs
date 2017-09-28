#![feature(entry_or_default)]

#[derive(Debug)]
struct TrieNode<K: std::cmp::Eq + std::hash::Hash, V> {
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
    fn put(&mut self, key_word: &[K], data: V) {
        if key_word.len() == 0 {
            self.data = Some(data);
        } else {
            self.children.entry(key_word[0]).or_default().put(&key_word[1..], data);
        }
    }

    fn get(&self, key_word: &[K]) -> Option<&V> {
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
            root_node: TrieNode::default()
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

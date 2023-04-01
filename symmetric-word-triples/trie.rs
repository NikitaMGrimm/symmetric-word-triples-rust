pub type TreeChildMap<Token, Trie> = DashMap<Token, Arc<Trie>, H>;

pub struct Trie<Token> {
    children: TreeChildMap<Token, Trie>,
    pub is_terminal: bool,
}

impl<Token> Trie<Token> {
    pub fn new() -> Trie {
        Trie {
            children: TreeChildMap<Token>::new(),
            is_terminal: false,
        }
    }

    pub fn insert(&mut self, word: &[Token]) {
        
    }
}

// A trie looks like this
//
//     root
//     /  \
//    a    b
//    |    |\
//    b    a r
//   /

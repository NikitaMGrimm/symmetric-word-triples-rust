struct Node<T> {
    children: Vec<Node>,
    parents: Vec<Node>,
    is_terminal: bool,
    value: T,
}

struct Trie {
    children: Vec<Trie>,
}



// A trie looks like this
// 
//     root
//     /  \
//    a    b
//    |    |\
//    b    a r
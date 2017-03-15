use super::list;

#[derive(Debug, PartialEq)]
pub enum Node {
    List(list::List<Node>),
    Float(i32),
    Atom(String),
    String(String),
    True,
    False,
}

impl Node {
    /// Constuct a new True node
    ///
    pub fn true_() -> Self {
        Node::True
    }

    /// Constuct a new False node
    ///
    pub fn false_() -> Self {
        Node::False
    }

    /// Constuct a new Atom node
    ///
    pub fn atom(s: String) -> Self {
        Node::Atom(s)
    }

    /// Constuct a new Float node
    ///
    pub fn float(f: i32) -> Self {
        Node::Float(f)
    }

    /// Constuct a new string node
    ///
    pub fn string(s: String) -> Self {
        Node::String(s)
    }

    /// Constuct a new List node
    ///
    pub fn list(l: list::List<Node>) -> Self {
        Node::List(l)
    }

    /// Constuct a new List node from a vector of nodes
    ///
    pub fn list_from_vec(l: Vec<Node>) -> Self {
        Node::List(list::List::from_vec(l))
    }
}

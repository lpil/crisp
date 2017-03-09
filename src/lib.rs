pub mod list;
pub mod parser;

#[derive(Debug, PartialEq)]
pub enum Node {
    List(Vec<Node>),
    Float(i32),
    Atom(String),
    True,
    False,
}

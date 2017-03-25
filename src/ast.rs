use std::fmt::{self, Write};
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


    /// Write the AST to the buffer in the Crisp syntax.
    ///
    pub fn print(&self, buffer: &mut String) -> Result<(), fmt::Error> {
        match *self {
            Node::True => write!(buffer, "true"),
            Node::False => write!(buffer, "false"),
            Node::Float(f) => write!(buffer, "{}", f),
            Node::String(ref s) => write!(buffer, "{:?}", s),
            Node::Atom(ref a) => write!(buffer, "{}", a),
            Node::List(ref list) => print_list(buffer, list),
        }
    }
}


fn print_list(buffer: &mut String, list: &list::List<Node>) -> Result<(), fmt::Error> {
    try!(write!(buffer, "("));
    let mut iter = list.iter().peekable();
    while let Some(x) = iter.next() {
        x.print(buffer).unwrap();
        if iter.peek().is_some() {
            try!(write!(buffer, " "))
        }
    }
    write!(buffer, ")")
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::list::List;

    #[test]
    fn print_true() {
        let mut buffer = String::new();
        Node::true_().print(&mut buffer).unwrap();
        assert_eq!(buffer, "true".to_string())
    }

    #[test]
    fn print_false() {
        let mut buffer = String::new();
        Node::false_().print(&mut buffer).unwrap();
        assert_eq!(buffer, "false".to_string())
    }

    #[test]
    fn print_string() {
        let mut buffer = String::new();
        Node::string("Hello!".to_string()).print(&mut buffer).unwrap();
        assert_eq!(buffer, "\"Hello!\"".to_string())
    }

    #[test]
    fn print_float() {
        let mut buffer = String::new();
        Node::float(5).print(&mut buffer).unwrap();
        assert_eq!(buffer, "5".to_string())
    }

    #[test]
    fn print_atom() {
        let mut buffer = String::new();
        Node::atom("flat-map".to_string()).print(&mut buffer).unwrap();
        assert_eq!(buffer, "flat-map".to_string())
    }

    #[test]
    fn print_list_0() {
        let mut buffer = String::new();
        let list = List::new();
        Node::list(list).print(&mut buffer).unwrap();
        assert_eq!(buffer, "()".to_string())
    }

    #[test]
    fn print_list_1() {
        let mut buffer = String::new();
        let list = List::new().cons(Node::float(5));
        Node::list(list).print(&mut buffer).unwrap();
        assert_eq!(buffer, "(5)".to_string())
    }

    #[test]
    fn print_list_2() {
        let mut buffer = String::new();
        let list = List::new().cons(Node::float(5)).cons(Node::atom("-".to_string()));
        Node::list(list).print(&mut buffer).unwrap();
        assert_eq!(buffer, "(- 5)".to_string())
    }

    #[test]
    fn print_list_3() {
        let mut buffer = String::new();
        let list = List::new()
            .cons(Node::float(40))
            .cons(Node::float(5))
            .cons(Node::atom("-".to_string()));
        Node::list(list).print(&mut buffer).unwrap();
        assert_eq!(buffer, "(- 5 40)".to_string())
    }
}

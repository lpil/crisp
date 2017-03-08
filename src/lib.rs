pub mod parser;

#[derive(Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Div,
    Mult,
}


#[derive(Debug, PartialEq)]
pub enum Sexpr {
    List(Op, Vec<Sexpr>),
    Value(f64),
}

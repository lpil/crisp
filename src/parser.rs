use std::iter::Peekable;
use std::str;
use super::ast::Node;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ReservedChar,
    BadList,
}

enum ParseResult {
    Ok(Node),
    None,
    Err(ParseError),
}

/// Parse source code into an abstract syntax tree.
///
pub fn parse(input: &str) -> Result<Vec<Node>, ParseError> {
    let mut chars = input.chars().peekable();
    parse_nodes(&mut chars)
}

fn parse_nodes(mut chars: &mut Peekable<str::Chars>) -> Result<Vec<Node>, ParseError> {
    let mut nodes = vec![];
    loop {
        chomp(&mut chars);
        match parse_node(&mut chars) {
            ParseResult::Err(error) => return Err(error),
            ParseResult::Ok(node) => nodes.push(node),
            ParseResult::None => break,
        }
    }
    Ok(nodes)
}

fn parse_node(mut chars: &mut Peekable<str::Chars>) -> ParseResult {
    if starts_with_reserved_char(&mut chars) {
        return ParseResult::Err(ParseError::ReservedChar);
    }
    if let Some(atom) = parse_atom(&mut chars) {
        return ParseResult::Ok(atom);
    }
    if let Some(num) = parse_number(&mut chars) {
        return ParseResult::Ok(num);
    }
    parse_list(&mut chars)
}

fn parse_atom(chars: &mut Peekable<str::Chars>) -> Option<Node> {
    let mut buffer = String::new();
    if !valid_atom_start_char(chars) {
        return None;
    }
    while let Some(&c) = chars.peek() {
        if !c.is_whitespace() && !c.is_control() && c != '(' && c != ')' {
            buffer.push(c);
            chars.next();
        } else {
            break;
        }
    }
    match &*buffer {
        "true" => Some(Node::true_()),
        "false" => Some(Node::false_()),
        _ => Some(Node::atom(buffer)),
    }
}

fn starts_with_reserved_char(chars: &mut Peekable<str::Chars>) -> bool {
    match chars.peek() {
        Some(&'#') | Some(&'[') | Some(&']') | Some(&'{') | Some(&'}') | Some(&'"') |
        Some(&'\'') | Some(&'`') => true,
        _ => false,
    }
}

fn valid_atom_start_char(chars: &mut Peekable<str::Chars>) -> bool {
    !starts_with_reserved_char(chars) &&
    match chars.peek() {
        Some(&'(') | Some(&')') | None => false,
        Some(c) => !(c.is_whitespace() || c.is_control() || c.is_digit(10)),
    }
}

fn parse_number(chars: &mut Peekable<str::Chars>) -> Option<Node> {
    let mut point = false;
    let mut nums = String::new();
    while let Some(&c) = chars.peek() {
        if !point && c == '.' {
            point = true;
            nums.push(c);
            chars.next();
        } else if c.is_digit(10) {
            nums.push(c);
            chars.next();
        } else {
            break;
        }
    }
    match nums.parse() {
        Ok(n) => Some(Node::float(n)),
        Err(_) => None,
    }
}

fn parse_list(mut chars: &mut Peekable<str::Chars>) -> ParseResult {
    if chars.peek() != Some(&'(') {
        return ParseResult::None;
    }
    chars.next();
    chomp(&mut chars);
    let elements = match parse_nodes(&mut chars) {
        Ok(e) => e,
        Err(e) => return ParseResult::Err(e),
    };
    if chars.peek() == Some(&')') {
        chars.next();
        ParseResult::Ok(Node::list_from_vec(elements))
    } else {
        ParseResult::Err(ParseError::BadList)
    }
}

/// Drop preceeding spaces
///
fn chomp(chars: &mut Peekable<str::Chars>) {
    while let Some(&c) = chars.peek() {
        if c == ' ' {
            chars.next();
        } else {
            break;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::ParseError::*;
    use super::super::ast::Node;

    #[test]
    fn parse_test() {
        let input = "(+ 1 2)".to_string();
        let list =
            Node::list_from_vec(vec![Node::atom("+".to_string()), Node::float(1), Node::float(2)]);
        assert_eq!(parse(&input), Ok(vec![list]));
    }


    #[test]
    fn parse_top_level_values() {
        let input = "() 1 /".to_string();
        assert_eq!(parse(&input),
                   Ok(vec![Node::list_from_vec(vec![]),
                           Node::float(1),
                           Node::atom("/".to_string())]));
    }

    #[test]
    fn parse_list_empty() {
        assert_eq!(parse(&"".to_string()), Ok(vec![]));
    }

    #[test]
    fn parse_list_of_num() {
        assert_eq!(parse(&"(123)".to_string()),
                   Ok(vec![Node::list_from_vec(vec![Node::float(123)])]));
    }

    #[test]
    fn parse_incomplete_list() {
        assert_eq!(parse(&"(123".to_string()), Err(BadList));
    }


    #[test]
    fn parse_multi_num_list() {
        assert_eq!(parse(&"(1 2 3)".to_string()),
                   Ok(vec![Node::list_from_vec(vec![Node::float(1),
                                                    Node::float(2),
                                                    Node::float(3)])]));
    }

    #[test]
    fn parse_nested_list() {
        assert_eq!(parse(&"(1 (3))".to_string()),
                   Ok(vec![Node::list_from_vec(vec![Node::float(1),
                                                    Node::list_from_vec(vec![Node::float(3)])])]));
    }

    #[test]
    fn parse_number_1_digit() {
        assert_eq!(parse(&"5".to_string()), Ok(vec![Node::float(5)]));
    }

    #[test]
    fn parse_number_2_digits() {
        assert_eq!(parse(&"52".to_string()), Ok(vec![Node::float(52)]));
    }

    #[test]
    fn parse_atom_lowercase() {
        assert_eq!(parse(&"hello".to_string()),
                   Ok(vec![Node::atom("hello".to_string())]));
    }

    #[test]
    fn parse_atom_uppercase() {
        assert_eq!(parse(&"HELLO".to_string()),
                   Ok(vec![Node::atom("HELLO".to_string())]));
    }

    #[test]
    fn parse_atom_mixed_case() {
        assert_eq!(parse(&"HelLO".to_string()),
                   Ok(vec![Node::atom("HelLO".to_string())]));
    }

    #[test]
    fn parse_atom_with_dash() {
        assert_eq!(parse(&"hi-there".to_string()),
                   Ok(vec![Node::atom("hi-there".to_string())]));
    }

    #[test]
    fn parse_atom_with_underscope() {
        assert_eq!(parse(&"hi_there".to_string()),
                   Ok(vec![Node::atom("hi_there".to_string())]));
    }

    #[test]
    fn parse_atom_with_other_chars() {
        assert_eq!(parse(&"chars1234567890<~>!?\\/:;@#".to_string()),
                   Ok(vec![Node::atom("chars1234567890<~>!?\\/:;@#".to_string())]));
    }

    #[test]
    fn parse_atom_true() {
        assert_eq!(parse(&"true".to_string()), Ok(vec![Node::true_()]));
    }

    #[test]
    fn parse_atom_false() {
        assert_eq!(parse(&"false".to_string()), Ok(vec![Node::false_()]));
    }

    #[test]
    fn parse_atom_blacklisted_starts() {
        assert_eq!(parse(&mut "#".to_string()), Err(ReservedChar));
        assert_eq!(parse(&mut "[".to_string()), Err(ReservedChar));
        assert_eq!(parse(&mut "]".to_string()), Err(ReservedChar));
        assert_eq!(parse(&mut "{".to_string()), Err(ReservedChar));
        assert_eq!(parse(&mut "}".to_string()), Err(ReservedChar));
        assert_eq!(parse(&mut "'".to_string()), Err(ReservedChar));
        assert_eq!(parse(&mut "`".to_string()), Err(ReservedChar));
        assert_eq!(parse(&mut "\"".to_string()), Err(ReservedChar));
    }
}

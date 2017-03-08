use std::iter;
use std::str;
use super::Node;


pub fn parse(input: &String) -> Result<Node, String> {
    let mut chars = input.chars().peekable();
    parse_list(&mut chars)
}

pub fn parse_atom(chars: &mut iter::Peekable<str::Chars>) -> Result<Node, String> {
    let mut buffer = String::new();
    if !valid_atom_start_char(chars) {
        return Err("Invalid atom".to_string());
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
        "true" => Ok(Node::True),
        "false" => Ok(Node::False),
        _ => Ok(Node::Atom(buffer)),
    }
}

fn valid_atom_start_char(chars: &mut iter::Peekable<str::Chars>) -> bool {
    match chars.peek() {
        None => false,
        Some(&'(') | Some(&')') | Some(&'[') | Some(&']') | Some(&'{') | Some(&'}') |
        Some(&'"') | Some(&'\'') | Some(&'`') => false,
        Some(c) => !(c.is_whitespace() || c.is_control() || c.is_digit(10)),
    }
}

pub fn parse_number(chars: &mut iter::Peekable<str::Chars>) -> Result<Node, String> {
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
        Ok(n) => Ok(Node::Float(n)),
        Err(_) => Err("Invalid float".to_string()),
    }
}

pub fn parse_list(mut chars: &mut iter::Peekable<str::Chars>) -> Result<Node, String> {
    if chars.peek() != Some(&'(') {
        return Err("Invalid list. Expected `(`".to_string());
    }
    chars.next();
    chomp(&mut chars);
    let elements = parse_elems(&mut chars);
    if chars.peek() == Some(&')') {
        chars.next();
        Ok(Node::List(elements))
    } else {
        Err("Invalid list. Expected `)`".to_string())
    }
}

fn parse_elems(mut chars: &mut iter::Peekable<str::Chars>) -> Vec<Node> {
    let mut elems = vec![];
    loop {
        chomp(&mut chars);
        if let Ok(atom) = parse_atom(&mut chars) {
            elems.push(atom);
            continue;
        }
        if let Ok(num) = parse_number(&mut chars) {
            elems.push(num);
            continue;
        }
        if let Ok(list) = parse_list(&mut chars) {
            elems.push(list);
            continue;
        }
        break;
    }
    elems
}

/// Drop preceeding spaces
///
fn chomp(chars: &mut iter::Peekable<str::Chars>) {
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
    use super::super::Node::*;

    // Parse

    #[test]
    fn parse_test() {
        let input = "(+ 1 2)".to_string();
        let res = parse(&input);
        let nums = vec![Atom("+".to_string()), Float(1), Float(2)];
        let sexpr = List(nums);
        assert_eq!(res, Ok(sexpr));
    }

    // parse_list

    #[test]
    fn parse_list_empty() {
        let mut chars = "".chars().peekable();
        let res = parse_list(&mut chars);
        assert_eq!(res, Err("Invalid list. Expected `(`".to_string()));
    }

    #[test]
    fn parse_list_of_num() {
        let mut chars = "(123)".chars().peekable();
        let res = parse_list(&mut chars);
        let sexpr = List(vec![Float(123)]);
        assert_eq!(res, Ok(sexpr));
    }

    #[test]
    fn parse_incomplete_list() {
        let mut chars = "(123".chars().peekable();
        let res = parse_list(&mut chars);
        assert_eq!(res, Err("Invalid list. Expected `)`".to_string()));
    }


    #[test]
    fn parse_multi_num_list() {
        let mut chars = "(1 2 3)".chars().peekable();
        let res = parse_list(&mut chars);
        let nums = vec![Float(1), Float(2), Float(3)];
        let sexpr = List(nums);
        assert_eq!(res, Ok(sexpr));
    }

    #[test]
    fn parse_nested_list() {
        let mut chars = "(1 (3))".chars().peekable();
        let res = parse_list(&mut chars);
        let sexpr1 = List(vec![Float(3)]);
        let sexpr2 = List(vec![Float(1), sexpr1]);
        assert_eq!(res, Ok(sexpr2));
    }

    // parse_number

    #[test]
    fn parse_number_empty() {
        let mut chars = "".chars().peekable();
        let res = parse_number(&mut chars);
        assert_eq!(res, Err("Invalid float".to_string()));
    }

    #[test]
    fn parse_number_invalid() {
        let mut chars = "o".chars().peekable();
        let res = parse_number(&mut chars);
        assert_eq!(res, Err("Invalid float".to_string()));
    }

    #[test]
    fn parse_number_digit_then_letter() {
        let mut chars = "11o".chars().peekable();
        let res = parse_number(&mut chars);
        assert_eq!(res, Ok(Float(11)));
        assert_eq!(chars.peek(), Some(&'o'));
    }

    #[test]
    fn parse_number_1_digit() {
        let mut chars = "5".chars().peekable();
        let res = parse_number(&mut chars);
        assert_eq!(res, Ok(Float(5)));
    }

    #[test]
    fn parse_number_2_digits() {
        let mut chars = "52".chars().peekable();
        let res = parse_number(&mut chars);
        assert_eq!(res, Ok(Float(52)));
    }

    #[test]
    fn parse_number_3_digits() {
        let mut chars = "524   ".chars().peekable();
        let res = parse_number(&mut chars);
        assert_eq!(res, Ok(Float(524)));
    }

    // parse_atom

    #[test]
    fn parse_atom_lowercase() {
        let mut chars = "hello".chars().peekable();
        let res = parse_atom(&mut chars);
        assert_eq!(res, Ok(Atom("hello".to_string())));
    }

    #[test]
    fn parse_atom_uppercase() {
        let mut chars = "HELLO".chars().peekable();
        let res = parse_atom(&mut chars);
        assert_eq!(res, Ok(Atom("HELLO".to_string())));
    }

    #[test]
    fn parse_atom_mixed_case() {
        let mut chars = "HelLO".chars().peekable();
        let res = parse_atom(&mut chars);
        assert_eq!(res, Ok(Atom("HelLO".to_string())));
    }

    #[test]
    fn parse_atom_with_dash() {
        let mut chars = "hi-there".chars().peekable();
        let res = parse_atom(&mut chars);
        assert_eq!(res, Ok(Atom("hi-there".to_string())));
    }

    #[test]
    fn parse_atom_with_underscope() {
        let mut chars = "what_up".chars().peekable();
        let res = parse_atom(&mut chars);
        assert_eq!(res, Ok(Atom("what_up".to_string())));
    }

    #[test]
    fn parse_atom_with_other_chars() {
        let mut chars = "chars1234567890<~>!?\\/:;@#".chars().peekable();
        let res = parse_atom(&mut chars);
        assert_eq!(res, Ok(Atom("chars1234567890<~>!?\\/:;@#".to_string())));
    }

    #[test]
    fn parse_atom_true() {
        let mut chars = "true".chars().peekable();
        let res = parse_atom(&mut chars);
        assert_eq!(res, Ok(True));
    }

    #[test]
    fn parse_atom_false() {
        let mut chars = "false".chars().peekable();
        let res = parse_atom(&mut chars);
        assert_eq!(res, Ok(False));
    }

    #[test]
    fn parse_atom_blacklisted_starts() {
        assert!(parse_atom(&mut "0".chars().peekable()).is_err());
        assert!(parse_atom(&mut "1".chars().peekable()).is_err());
        assert!(parse_atom(&mut "2".chars().peekable()).is_err());
        assert!(parse_atom(&mut "3".chars().peekable()).is_err());
        assert!(parse_atom(&mut "4".chars().peekable()).is_err());
        assert!(parse_atom(&mut "5".chars().peekable()).is_err());
        assert!(parse_atom(&mut "6".chars().peekable()).is_err());
        assert!(parse_atom(&mut "7".chars().peekable()).is_err());
        assert!(parse_atom(&mut "8".chars().peekable()).is_err());
        assert!(parse_atom(&mut "9".chars().peekable()).is_err());
        assert!(parse_atom(&mut "(".chars().peekable()).is_err());
        assert!(parse_atom(&mut ")".chars().peekable()).is_err());
        assert!(parse_atom(&mut "[".chars().peekable()).is_err());
        assert!(parse_atom(&mut "]".chars().peekable()).is_err());
        assert!(parse_atom(&mut "{".chars().peekable()).is_err());
        assert!(parse_atom(&mut "}".chars().peekable()).is_err());
        assert!(parse_atom(&mut "'".chars().peekable()).is_err());
        assert!(parse_atom(&mut "`".chars().peekable()).is_err());
        assert!(parse_atom(&mut "\"".chars().peekable()).is_err());
    }
}

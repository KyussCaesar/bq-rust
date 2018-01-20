//! The parser builds up a query in memory as a query object.
//!
//! Query objects are a tree made up of nodes and allow the query to be matched against the text
//! using relatively simple rules.
//!
//! # Matching rules:
//!
//! A query is built up as a tree.
//! A query is considered to match the text if all nodes in the tree match.
//!
//! * A leaf node matches if the string associated with the node is in the text.
//! * An AND node matches if *both* its left *and* right children match.
//! * An OR node matches if *either* its left *or* right children match.
//! * A NOT node matches if it's child node *doesn't* match.

use std::collections::VecDeque;

/// Represents a node in the internal string.
pub enum Node
{
    AND(Box<Node>, Box<Node>),
    OR(Box<Node>, Box<Node>),
    NOT(Box<Node>),

    /// The Leaf nodes are the keywords to search for in the text.
    ///
    /// String: The keyword to search for.
    /// Vec<i64>: The (precomputed) jump table for the Knuth-Morris-Pratt algorithm.
    Leaf(String, Vec<i64>)
}

#[derive(Debug)]
enum Token
{
    AND,
    OR,
    NOT,
    LParen,
    RParen,
    Keyword(String),
}

/// Represents an error in parsing the query.
#[derive(Debug)] pub struct ParsingError(&'static str);

// pub enum Result<T>
// {
//     Ok(T),
//     Err(ParsingError),
// }

pub type Result<T> = ::std::result::Result<T, ParsingError>;

/// Constructs a new query object from string.
///
/// Returns a ParsingError if the query is malformed.
pub fn from(s: &str) -> Result<Node>
{
    match tokenise_query(s.to_string())
    {
        Ok(ts) => return build_bquery(ts),
        Err(e) => return Err(e),
    }
}

fn tokenise_query(query: String) -> Result<VecDeque<Token>>
{
    let mut query = query.chars();
    let mut tokens: VecDeque<Token> = VecDeque::new();

    // persistent state
    let mut quotes = false;
    let mut current_token = String::new();

    while let Some(c) = query.next()
    {
        if quotes
        {
            if c == '"'
            {
                tokens.push_back(Token::Keyword(current_token));
                current_token = String::new();
                quotes = false;
            }

            else { current_token.push(c); }
        }

        else
        {
            match c
            {
                '"' =>
                {
                    current_token = String::new();
                    quotes = true;
                },

                'a'...'z' | 'A'...'Z' =>
                {
                    return Err(ParsingError("Found an alphabetic character when either a quote, parenthesis, or operator was expected"));
                },

                '&' => tokens.push_back(Token::AND),
                '|' => tokens.push_back(Token::OR),
                '!' => tokens.push_back(Token::NOT),
                '(' => tokens.push_back(Token::LParen),
                ')' => tokens.push_back(Token::RParen),

                // skip whitespace
                ' ' | '\t' | '\n' | '\r' => continue,

                _ => return Err(ParsingError("found an unexpected character")),
            }
        }
    }

    return Ok(tokens);
}

fn build_bquery(mut tokens: VecDeque<Token>) -> Result<Node>
{
    return build_query(&mut tokens);
}

fn build_query(tokens: &mut VecDeque<Token>) -> Result<Node>
{
    match build_or_group(tokens)
    {
        Ok(mut left) =>
        {
            while let Some(t) = tokens.pop_front()
            {
                if let Token::OR = t
                {
                    match build_or_group(tokens)
                    {
                        Ok(mut right) => left = Node::OR(Box::new(left), Box::new(right)),
                        Err(e) => return Err(e),
                    }
                }

                else
                {
                    tokens.push_front(t);
                    return Ok(left);
                }
            }

            return Ok(left);
        },

        Err(e) => return Err(e),
    }
}

fn build_or_group(tokens: &mut VecDeque<Token>) -> Result<Node>
{
    match build_and_group(tokens)
    {
        Ok(mut left) =>
        {
            while let Some(t) = tokens.pop_front()
            {
                if let Token::AND = t
                {
                    match build_and_group(tokens)
                    {
                        Ok(mut right) => left = Node::AND(Box::new(left), Box::new(right)),
                        Err(e) => return Err(e),
                    }
                }

                else
                {
                    tokens.push_front(t);
                    return Ok(left);
                }
            }

            return Ok(left);
        },

        Err(e) => return Err(e),
    }
}

fn build_and_group(tokens: &mut VecDeque<Token>) -> Result<Node>
{
    use self::Token::*;

    if let Some(t) = tokens.pop_front()
    {
        match t
        {
            NOT =>
            {
                match build_and_group(tokens)
                {
                    Ok(node) => return Ok(Node::NOT(Box::new(node))),
                    Err(e) => return Err(e),
                }
            },

            Keyword(s) =>
            {
                let table = kmp_table(&s.clone().into_bytes());
                return Ok(Node::Leaf(s, table));
            },

            LParen =>
            {
                let expr = match build_query(tokens)
                {
                    Ok(expr) => expr,
                    Err(e) => return Err(e),
                };

                if let Some(Token::RParen) = tokens.pop_front()
                {
                    return Ok(expr);
                }

                else
                {
                    return Err(ParsingError("Expected closing parentheses after expression"));
                }
            },

            _ => return Err(ParsingError("Unexpected token ")),
        }
    }

    else
    {
        return Err(ParsingError("Unexpected end of input"));
    }
}

/// Computes the jump table for the Knuth-Morris-Pratt algorithm.
fn kmp_table(s1: &Vec<u8>) -> Vec<i64>
{
    let mut i: i64 = 1;
    let mut j: i64 = -1;

    // Using {vec::with_capacity, vec::reserve, vec::reserve_exact} didn't work, so this is what I
    // resorted to.
    let mut next: Vec<i64> = Vec::new();
    for _ in s1
    {
        next.push(0);
    }

    next[0] = -1;

    while i < s1.len() as i64
    {
        while (j > -1) && (s1[(j+1) as usize] != s1[(j+1) as usize])
        {
            j = next[j as usize];
        }

        if s1[i as usize] == s1[(j+1) as usize]
        {
            j += 1;
        }

        next[i as usize] = j;

        i += 1;
    }

    return next;
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn do_both(s: String)
    {
        build_bquery(tokenise_query(s).unwrap()).unwrap();
    }

    #[test]
    fn test()
    {
        do_both("\"iphone\"".to_string());
    }

    #[test]
    #[should_panic]
    fn test2()
    {
        do_both("iphone".to_string());
    }

    #[test]
    fn whitespace()
    {
        do_both("\"iphone\" | \"i phone\"".to_string());
    }
}

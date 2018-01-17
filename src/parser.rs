use std::collections::VecDeque;

pub enum Node
{
    AND(Box<Node>, Box<Node>),
    OR(Box<Node>, Box<Node>),
    NOT(Box<Node>),
    Leaf(String)
}

enum Token
{
    AND,
    OR,
    NOT,
    LParen,
    RParen,
    Keyword(String),
}

pub fn from(s: String) -> Node
{
    return build_bquery(tokenise_query(s));
}

fn tokenise_query(query: String) -> VecDeque<Token>
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
                    panic!("found a letter when none was expected");
                },

                '&' => tokens.push_back(Token::AND),
                '|' => tokens.push_back(Token::OR),
                '!' => tokens.push_back(Token::NOT),
                '(' => tokens.push_back(Token::RParen),
                ')' => tokens.push_back(Token::LParen),

                // skip whitespace
                ' ' | '\t' | '\n' | '\r' => continue,

                _ => panic!("found an unexpected character: {}", c),
            }
        }
    }

    return tokens;
}

fn build_bquery(mut tokens: VecDeque<Token>) -> Node
{
    return build_expression(&mut tokens);
}

fn build_expression(tokens: &mut VecDeque<Token>) -> Node
{
    let mut left = build_term(tokens);
    while let Some(t) = tokens.pop_front()
    {
        if let Token::OR = t
        {
            let right = build_term(tokens);
            left = Node::OR(Box::new(left), Box::new(right));
        }
        
        else
        {
            tokens.push_front(t);
            return left;
        }
    }

    return left;
}

fn build_term(tokens: &mut VecDeque<Token>) -> Node
{
    let mut left = build_factor(tokens);
    while let Some(t) = tokens.pop_front()
    {
        if let Token::AND = t
        {
            let right = build_factor(tokens);
            left = Node::AND(Box::new(left), Box::new(right));
        }

        else
        {
            tokens.push_front(t);
            return left;
        }
    }

    return left;
}

fn build_factor(tokens: &mut VecDeque<Token>) -> Node
{
    use self::Token::*;
    if let Some(t) = tokens.pop_front()
    {
        match t
        {
            NOT => return Node::NOT(Box::new(build_factor(tokens))),
            Keyword(s) => return Node::Leaf(s),
            LParen =>
            {
                let expr = build_expression(tokens);
                if let Some(Token::RParen) = tokens.pop_front()
                {
                    return expr;
                }

                else
                {
                    panic!("ERROR! Expected closing parentheses after expression");
                }
            },

            _ => panic!("ERROR! Unexpected character"),
        }
    }

    else
    {
        panic!("ERROR! Unexpected end of input");
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn do_both(s: String)
    {
        build_bquery(tokenise_query(s));
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

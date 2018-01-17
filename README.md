# bq-rust

This is a library for applying a boolean query to text.

## Example:

``` rust
extern crate bq_rust;

use bq_rust::Matcher;

let greeting = Matcher::from("\"hello\" | \"hi\"");
assert!(greeting.query("hi there!"));
assert!(greeting.query("hello i am here"));

let greet2 = Matcher::from("(\"hello\" | \"hi\" ) \"there\"");
assert!(greet2.query("hello there!"));
assert!(greet2.query("hi there!"));
```

## Usage

Use `Matcher::from` to create a match object.

Use the `query` method of match objects to find out if the text is a match for the query.

## Description

Internally, the query is represented in a tree structure. The interior nodes are the operators AND, OR and NOT, and the leaf nodes are the keywords for the search.

The library checks for any occurence of each keyword in the text, and decides whether to return TRUE or FALSE dependent on the interior nodes.

Uses an implementation of the Knuth-Morris-Pratt algorithm (that I shamelessly copied from a C implementation I found on the internet) for fast string matching.

## Syntax

### In pseudo-BNF language:

```
query:
    or-group ( '|' or-group )*

or-group:
    and-group ( '&' and-group )*

and-group:
    STRINGLITERAL
    '!' and-group
    '(' query ')'
```

Where `STRINGLITERAL` is any ASCII character enclosed in double quotes.

### In English

Use '&' for AND, '|' for OR, '!' for NOT. NOT has highest precedence, then AND, then OR.

You can use '(' and ')' for grouping.

```
let complexq = Matcher::from(" \"that\" & ( \"this\" | \"these\" ) & \"those\" ");
assert!(complexq.query("that and this with those"));
assert!(complexq.query("that and these with those"));
```

## Issues

When parsing the boolean query, the parser will `panic!` if there is an error, rather than simply reporting to the user.

## Next steps

* Case-insensitive matching.
* Fuzzy-matching?

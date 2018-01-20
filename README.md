# bqrs

This is a library for applying a boolean filter to text.

visit me on [crates.io](https://crates.io/crates/bqrs)

## Example:

``` rust
extern crate bq_rust;

use bq_rust::Matcher;

let greeting = Matcher::from("'hello' | 'hi'");
assert!(greeting.query("hi there!"));
assert!(greeting.query("hello i am here"));

let greet2 = Matcher::from("('hello' | 'hi' ) 'there'");
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

``` bnf
query:
    or-group ( '|' or-group )*

or-group:
    and-group ( '&' and-group )*

and-group:
    STRINGLITERAL
    '!' and-group
    '(' query ')'
```

Where `STRINGLITERAL` is any ASCII character sequence enclosed in quotes.

### In English

Use '&' for AND, '|' for OR, '!' for NOT. NOT has highest precedence, then AND, then OR.

You can use '(' and ')' for grouping.

``` rust
let complexq = Matcher::from(" 'that' & ( 'this' | 'these' ) & 'those' ");
assert!(complexq.query("that and this with those"));
assert!(complexq.query("that and these with those"));
```

## TODOs

* Single-quotes.
* Case-insensitive matching.
* Documentation.
* More tests.
* Visualise query as a graph by outputting dot-language files.

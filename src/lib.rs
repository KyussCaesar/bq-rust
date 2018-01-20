mod parser;
mod matcher;

/// The main interface with this library is via Matcher objects.
///
/// # Examples:
///
/// ``` rust
/// use bqrs::Matcher;
///
/// // this matcher matches any text that contains the words
/// // 'these', 'those' and either 'this' or 'that'
/// let myquery = Matcher::from("(\"this\" | \"that\") & \"these\" & \"those\"").unwrap();
/// assert!(myquery.query("this these those"));
/// assert!(myquery.query("that these those"));
///
/// // doesn't contain 'those'
/// assert!(!myquery.query("this that these"));
/// ```
pub use matcher::Matcher;

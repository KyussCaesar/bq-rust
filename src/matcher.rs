use parser;

/// The matcher struct holds a single query object.
pub struct Matcher
{
    query: parser::Node
}

impl Matcher
{
    /// Constructs a new Matcher object from a string.
    ///
    /// Returns a ParsingError if it fails.
    pub fn from(s: &str) -> parser::Result<Self>
    {
        match parser::from(s)
        {
            Ok(q) => return Ok(Matcher { query: q }),
            Err(e) => return Err(e),
        }
    }

    /// Applies the query to the string.
    pub fn query(&self, s: &str) -> bool
    {
        return match_bquery(&self.query, s)
    }
}

/// Applies `query` to `s`.
fn match_bquery(query: &parser::Node, s: &str) -> bool
{
    use parser::Node::*;
    match query
    {
        &AND(ref a, ref b) => return match_bquery(&*a, s) && match_bquery(&*b, s),
        &OR(ref a, ref b) => return match_bquery(&*a, s) || match_bquery(&*b, s),
        &NOT(ref a) => return !match_bquery(&*a, s),
        &Leaf(ref keyword, ref jumptable) => return kmp(jumptable, keyword, s),
    }
}

/// An implementation of the [Knuth-Morris-Pratt](https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm) algorithm.
/// I didn't come up with this, it is taken from a C implementation that I found elsewhere.
///
/// Parameters:
/// `table`: The precomputed jump table.
/// `s1`: The string to search for.
/// `s2`: The text to search for s1 in.
fn kmp(table: &Vec<i64>, s1: &str, s2: &str) -> bool
{
    let s1 = s1
        .to_string()
        .to_lowercase()
        .into_bytes();

    let s2 = s2
        .to_string()
        .to_lowercase()
        .into_bytes();

    let mut i: i64 = 0;
    let mut j: i64 = -1;

    while i < s2.len() as i64
    {
        while (j > -1) & (s1[(j+1) as usize] != s2[i as usize])
        {
            j = table[j as usize];
        }

        if s2[i as usize] == s1[(j+1) as usize]
        {
            j += 1;
        }

        if j == (s1.len()as i64 -1)
        {
            return true;
        }

        i += 1;
    }

    return false;
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn print_on_failure(m: &Matcher, s: &str)
    {
        assert!(m.query(s), "was trying to match the string '{}'", s);
    }

    #[test]
    fn test()
    {
        let iphonex = Matcher::from("\"iphone\" | \"i phone\"").unwrap();
        print_on_failure(&iphonex, "I love my new iphone!");
    }

    #[test]
    fn groups()
    {
        let greeting = Matcher::from("(\"hello\" | \"hi\") & \"there\")").unwrap();
        print_on_failure(&greeting, "hi there, my name is Kyuss Caesar");
        print_on_failure(&greeting, "hello there, this should also be a greeting");
    }

    #[test]
    fn casesens()
    {
        let greeting = Matcher::from("('hello'|'hi'|'ho') & 'there'").unwrap();
        print_on_failure(&greeting, "HELLO THERE");
        print_on_failure(&greeting, "hI THERE");
        print_on_failure(&greeting, "Hi there!");
    }
}

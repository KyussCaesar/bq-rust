use parser;

pub struct Matcher
{
    query: parser::Node
}

impl Matcher
{
    pub fn from(s: &str) -> Self
    {
        Matcher
        {
            query: parser::from(s)
        }
    }

    pub fn query(&self, s: &str) -> bool
    {
        return match_bquery(&self.query, s)
    }
}

fn match_bquery(query: &parser::Node, s: &str) -> bool
{
    use parser::Node::*;
    match query
    {
        &AND(ref a, ref b) => return match_bquery(&*a, s) && match_bquery(&*b, s),
        &OR(ref a, ref b) => return match_bquery(&*a, s) || match_bquery(&*b, s),
        &NOT(ref a) => return !match_bquery(&*a, s),
        &Leaf(ref keyword) => return kmp(keyword, s),
    }
}

// implementation of knuth-morris-pratt
// returns true if s1 is in s2
fn kmp(s1: &str, s2: &str) -> bool
{
    let s1 = s1.to_string().into_bytes();
    let s2 = s2.to_string().into_bytes();

    let table = kmp_table(&s1);

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

fn kmp_table(s1: &Vec<u8>) -> Vec<i64>
{
    let mut i: i64 = 1;
    let mut j: i64 = -1;

    let mut next: Vec<i64> = Vec::new();
    for _num in s1
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

    fn print_on_failure(m: &Matcher, s: &str)
    {
        assert!(m.query(s), "was trying to match the string '{}'", s);
    }

    #[test]
    fn test()
    {
        let iphonex = Matcher::from("\"iphone\" | \"i phone\"");
        print_on_failure(&iphonex, "I love my new iphone!");
    }

    #[test]
    fn groups()
    {
        let greeting = Matcher::from("(\"hello\" | \"hi\") & \"there\")");
        print_on_failure(&greeting, "hi there, my name is Kyuss Caesar");
        print_on_failure(&greeting, "hello there, this should also be a greeting");
    }
}

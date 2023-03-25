use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^https{0,1}://").unwrap();
}

pub fn is_remote<S>(s: S) -> bool
where
    S: AsRef<str>,
{
    RE.is_match(s.as_ref())
}

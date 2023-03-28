/// Encode URI by converting certain characters to
/// escape sequences representing the UTF-8 encoding
/// Standard: https://datatracker.ietf.org/doc/html/rfc3986
pub fn encode_rfc3986<S>(s: S) -> String
where
    S: AsRef<str>,
{
    let mut result = String::new();

    for i in s.as_ref().as_bytes().iter() {
        match i.to_owned() as char {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                result.push(i.to_owned() as char);
            }
            _ => result.push_str(format!("%{:02X}", i).as_str()),
        };
    }

    result
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_encode_rfc3986() {
        assert_eq!(super::encode_rfc3986("example.com"), "example.com");
        assert_eq!(super::encode_rfc3986("example.com/a"), "example.com%2Fa");
        assert_eq!(
            super::encode_rfc3986("http://exAmple.com:8000/a"),
            "http%3A%2F%2FexAmple.com%3A8000%2Fa"
        );
        assert_eq!(
            super::encode_rfc3986("http://auth@example.com/path/to/res?query_string=abc&def=123"),
            "http%3A%2F%2Fauth%40example.com%2Fpath%2Fto%2Fres%3Fquery_string%3Dabc%26def%3D123"
        );
        assert_eq!(
            super::encode_rfc3986("http://[::1]:8000/a%b%c!'()*"),
            "http%3A%2F%2F%5B%3A%3A1%5D%3A8000%2Fa%25b%25c%21%27%28%29%2A"
        );
        assert_eq!(
            super::encode_rfc3986("example.com/汉字/漢字"),
            "example.com%2F%E6%B1%89%E5%AD%97%2F%E6%BC%A2%E5%AD%97"
        );
    }
}

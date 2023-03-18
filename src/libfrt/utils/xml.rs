pub fn escape_str<S>(s: S) -> String
where
    S: AsRef<str>,
{
    let orig = s.as_ref();
    let mut s = String::new();
    s.reserve(orig.len());

    for i in orig.chars() {
        match i {
            '\"' => s.push_str("&quot;"),
            '\'' => s.push_str("&apos;"),
            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),
            '&' => s.push_str("&amp;"),
            _ => s.push(i),
        };
    }

    s
}

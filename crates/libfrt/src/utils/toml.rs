use toml::Value;

/// Merge two `toml::Value` to one
/// Patch `a` with `b`. like a.update(b) in Python
/// 
/// ```
/// let mut a = std::fs::read_to_string("1.toml").unwrap().parse::<Value>().unwrap();
/// let mut b = std::fs::read_to_string("2.toml").unwrap().parse::<Value>().unwrap();
/// 
/// merge(&mut a, b);
/// 
/// let s = T::deserialize(a).unwrap();
/// ```
pub fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Table(_), Value::Table(b)) => {
            let a = a.as_table_mut().unwrap();
            for (k, v) in b {
                let x = a.get_mut(&k);
                match x {
                    Some(ov) => merge(ov, v),
                    None => {
                        a.insert(k, v);
                        ()
                    }
                }
                //merge(a.entry(k).or_insert(Value::String(String::new())), v);
            }
        }
        (a, b) => *a = b,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_merge() {
        use toml::Value;
        let mut a = r#"
            a = "a"
            b = "b"
            c = "c"
            [d]
            q = "q"
            w = [ 1, 2, 3 ]
            [d.e]
            e = "e"
        "#.parse::<Value>().unwrap();
        let b = r#"
            a = "aa"
            [c]
            c = "cc"
            [d]
            t = "tt"
            w = [ "ww", "www" ]
            [d.e]
            e = "ee"
            [d.r]
            r = "rr"
        "#.parse::<Value>().unwrap();
        let expect = r#"
            a = "aa"
            b = "b"
            [c]
            c = "cc"
            [d]
            q = "q"
            t = "tt"
            w = [ "ww", "www" ]
            [d.e]
            e = "ee"
            [d.r]
            r = "rr"
        "#.parse::<Value>().unwrap();
        
        super::merge(&mut a, b);
        
        assert_eq!(a, expect)
    }
}
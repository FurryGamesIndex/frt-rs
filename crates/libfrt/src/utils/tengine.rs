use std::{borrow::Cow, collections::HashMap};

use anyhow::Result;
use regex::Regex;

pub fn simple_template_render(template: &str, context: &HashMap<String, String>) -> Result<String> {
    let mut s = template.to_owned();

    for (k, v) in context.iter() {
        let regex = Regex::new(format!("\\{{\\{{ *{} *\\}}\\}}", k).as_str())?;
        let result = regex.replace_all(&s, v);
        if let Cow::Owned(_) = result {
            s = result.into_owned();
        }
    }

    Ok(s)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_simple_template_render() {
        let context = HashMap::from([
            (String::from("_1"), String::from("aaa")),
            (String::from("_2"), String::from("bbb")),
            (String::from("_3"), String::from("ccc")),
            (String::from("vv"), String::from("ddd")),
            (String::from("bb"), String::from("eee")),
        ]);

        assert_eq!(
            super::simple_template_render("http://example.com/xxx", &context).unwrap(),
            "http://example.com/xxx"
        );
        assert_eq!(
            super::simple_template_render("http://example.com/{{ vv }}", &context).unwrap(),
            "http://example.com/ddd"
        );
        assert_eq!(
            super::simple_template_render("http://example.com/{{ _1 }}/{{_2}}", &context).unwrap(),
            "http://example.com/aaa/bbb"
        );
        assert_eq!(
            super::simple_template_render("http://example.com/{{ vv }}/{{bb}}", &context).unwrap(),
            "http://example.com/ddd/eee"
        );
        assert_eq!(
            super::simple_template_render("http://example.com/{{ _3 }}/{{ bb }}", &context)
                .unwrap(),
            "http://example.com/ccc/eee"
        );
    }
}

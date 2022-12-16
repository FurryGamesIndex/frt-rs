use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct HtmlText {
    pub html: String,
    pub plain: String,
}

impl<> From<String> for HtmlText {
    fn from(s: String) -> Self {
        Self {
            html: tera::escape_html(s.as_str()).replace("\n", "<br />"),
            plain: s
        }
    }
}

impl HtmlText {
    
}
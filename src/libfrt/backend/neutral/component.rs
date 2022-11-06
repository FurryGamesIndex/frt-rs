
pub trait Text {
    fn text(&self) -> String;
    fn raw_text(&self) -> String;
    fn raw_text_ref(&self) -> &str;
}


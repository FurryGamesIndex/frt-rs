#[derive(Debug)]
pub enum LangId {
    EnUs,
    ZhCn,
    ZhTw,
    Ja
}

impl From<&str> for LangId {
    fn from(s: &str) -> Self {
        match s {
            "zh-cn" | "zh" => LangId::ZhCn,
            "zh-tw" => LangId::ZhTw,
            "ja-jp" | "ja" => LangId::Ja,
            _ => LangId::EnUs,
        }
    }
}

impl LangId {
    pub fn as_unix(&self) -> &'static str {
        match self {
            LangId::EnUs => "en_US",
            LangId::ZhCn => "zh_CN",
            LangId::ZhTw => "zh_TW",
            LangId::Ja => "ja",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LangId::EnUs => "en-us",
            LangId::ZhCn => "zh-cn",
            LangId::ZhTw => "zh-tw",
            LangId::Ja => "ja",
        }
    }

    pub fn as_str_without_region(&self) -> &'static str {
        match self {
            LangId::EnUs => "en",
            LangId::ZhCn | LangId::ZhTw => "zh",
            LangId::Ja => "ja",
        }
    }
}
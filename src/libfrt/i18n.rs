use serde::Deserialize;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum LangId {
    EnUs,
    ZhCn,
    ZhTw,
    Ja
}

impl Default for LangId {
    fn default() -> Self {
        LangId::EnUs
    }
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

impl<'de> Deserialize<'de> for LangId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Ok(s.as_str().into())
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
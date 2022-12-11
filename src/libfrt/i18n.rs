use serde::Deserialize;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum LangId {
    EnUs,
    ZhCn,
    ZhTw,
    JaJp,
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
            "ja-jp" | "ja" => LangId::JaJp,
            _ => LangId::EnUs,
        }
    }
}

impl<'de> Deserialize<'de> for LangId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(s.as_str().into())
    }
}

impl LangId {
    pub fn tag(&self) -> (&'static str, &'static str, bool) {
        match self {
            LangId::EnUs => ("en", "us", true),
            LangId::ZhCn => ("zh", "cn", false),
            LangId::ZhTw => ("zh", "tW", false),
            LangId::JaJp => ("ja", "jp", true),
        }
    }

    pub fn as_unix(&self) -> String {
        let (l, r, _) = self.tag();
        format!("{}_{}", l, r.to_uppercase())
    }

    pub fn as_bcp47(&self) -> String {
        let (l, r, _) = self.tag();
        format!("{}-{}", l, r.to_uppercase())
    }

    pub fn as_bcp47_short(&self) -> String {
        let (l, r, s) = self.tag();
        if s {
            l.into()
        } else {
            format!("{}-{}", l, r.to_uppercase())
        }
    }

    pub fn as_str(&self) -> String {
        let (l, r, s) = self.tag();
        if s {
            l.into()
        } else {
            format!("{}-{}", l, r)
        }
    }

    pub fn as_str_noregion(&self) -> &'static str {
        self.tag().0
    }
}

use std::{collections::HashMap, rc::Rc};

use regex::Regex;

use crate::i18n::LangId;

pub struct StockLinkRule {
    pub name: String,

    pub icon: String,
    pub label: HashMap<LangId, String>,
    pub regex: Regex,
    pub named_matches: Vec<String>,
    pub www_href: String,
    pub inference: bool,
    pub passthrough: bool
}

pub struct Link {
    pub label: HashMap<LangId, String>,
    pub uri: String,
    pub rule_name: String,
    pub variables: HashMap<String, String>,
}
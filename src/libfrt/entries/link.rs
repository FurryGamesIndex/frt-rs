use std::collections::HashMap;
use std::rc::Rc;

use anyhow::Result;
use regex::Regex;
use serde::Deserialize;

use super::raw::RawLinkItem;
use crate::Context;
use crate::i18n::LangId;
use crate::error::{Error, ErrorKind};

#[derive(Deserialize, Default)]
pub struct StockLinkRule {
    pub name: String,
    pub icon: String,
    pub label: HashMap<LangId, String>,
    #[serde(with = "serde_regex")]
    pub regex: Option<Regex>,
    pub named_matches: Vec<String>,
    pub www_href: String,
    pub inference: bool,
    pub passthrough: bool
}

#[derive(Default)]
pub struct LinkRuleManager {
    pub rules: HashMap<String, Rc<StockLinkRule>>,
    pub inference_rules: Vec<(Regex, Rc<StockLinkRule>)>,
}

impl LinkRuleManager {
    pub fn add_rule(&mut self, rule: StockLinkRule) -> Result<()> {
        let rule = Rc::new(rule);

        self.rules.insert(rule.name.to_owned(), rule.clone());

        if !rule.passthrough && rule.inference {
            let x = rule.regex.as_ref();

            let regex = rule.regex.as_ref().ok_or_else(|| Error::new(
                ErrorKind::InvalidArgument,
                "StockLink: regex is required for inference rule"))?
            .clone();
            self.inference_rules.push((regex, rule.clone()));
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Link {
    pub label: HashMap<LangId, String>,
    pub uri: String,
    pub rule_name: String,
    pub variables: HashMap<String, String>,
}

impl Link {
    pub fn build(context: &Context, raw_link: RawLinkItem) -> Result<Self> {
        todo!()
    }
}
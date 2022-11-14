use std::collections::HashMap;
use std::rc::Rc;

use anyhow::Result;
use regex::Regex;
use serde::Deserialize;

use super::raw::RawLinkItem;
use crate::i18n::LangId;
use crate::error::{Error, ErrorKind};

#[derive(Debug)]
pub struct Link {
    pub label: HashMap<LangId, String>,
    pub uri: String,
    pub rule: Option<Rc<StockLinkRule>>,
    pub variables: HashMap<String, String>,
}

#[derive(Deserialize, Default, Debug)]
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

impl StockLinkRule {
    pub fn match_uri(&self, uri: &str) -> bool {
        match &self.regex {
            Some(regex) => regex.is_match(uri),
            None => true,
        }
    }

    pub fn build_link(&self, uri: &str) -> Result<Link> {
        todo!()
    }
}

#[derive(Default)]
pub struct LinkRuleManager {
    pub rules: HashMap<String, Rc<StockLinkRule>>,
    pub inference_rules: Vec<Rc<StockLinkRule>>,
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
            self.inference_rules.push(rule.clone());
        }

        Ok(())
    }

    pub fn build_link(&self, raw_link: RawLinkItem) -> Result<Link> {
        Ok(match raw_link {
            RawLinkItem::Custom { name, uri } => {
                if name.starts_with('.') { // stock link hint
                    let rule_name = &name[1..];
                    let rule = self.rules.get(rule_name)
                        .ok_or_else(|| Error::new(ErrorKind::NotExist, 
                            format!("Link rule '{rule_name}' not found").as_str()))?;

                    if !rule.match_uri(uri.as_str()) {
                        return Err(Error::new(ErrorKind::InvalidArgument, 
                            format!("URI '{uri}' not matchs rule '{rule_name}'").as_str()).into())
                    }

                    rule.build_link(uri.as_str())?
                } else {
                    Link {
                        label: HashMap::from([( LangId::default(), name )]),
                        uri,
                        rule: None,
                        variables: HashMap::new(),
                    }
                }
            },
            RawLinkItem::Auto(uri) => {
                let mut matched_rule = None;

                for rule in &self.inference_rules {
                    if rule.match_uri(uri.as_str()) {
                        matched_rule = Some(rule);
                        break;
                    }
                }

                match matched_rule {
                    Some(rule) => rule.build_link(uri.as_str())?,
                    None => return Err(Error::new(ErrorKind::NotExist, 
                        format!("Inference failed. No rule matchs '{uri}'.").as_str()).into()),
                }
            },
        })
    }
}
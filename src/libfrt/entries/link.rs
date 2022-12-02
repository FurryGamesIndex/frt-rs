use std::collections::HashMap;
use std::rc::Rc;

use anyhow::Result;
use regex::Regex;
use serde::Deserialize;
use crate::utils::tengine;

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
#[serde(default)]
pub struct StockLinkRule {
    pub name: String,
    pub icon: String,
    pub label: HashMap<LangId, String>,
    #[serde(with = "serde_regex")]
    pub regex: Option<Regex>,
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

    pub fn build_link(self: &Rc<Self>, uri: &str) -> Result<Link> {
        if self.passthrough {
            Ok(Link {
                label: self.label.clone(),
                uri: uri.to_owned(),
                rule: Some(self.clone()),
                variables: HashMap::new(),
            })
        } else {
            let mut label = self.label.clone();
            let mut final_uri = self.www_href.to_owned();
            let mut variables = HashMap::new();

            if let Some(re) = &self.regex {
                let caps = re.captures(uri)
                    .ok_or_else(|| Error::new(ErrorKind::InvalidArgument,
                        format!("Failed to parse URI '{}' by rule '{}': regex not match", uri, self.name).as_str()))?;

                let mut i = 0;
                for group in caps.iter() {
                    variables.insert(format!("_{i}"), group.unwrap().as_str().to_owned());
                    i += 1;
                }

                let named_variables: HashMap<String, String> = re
                    .capture_names()
                    .flatten()
                    .map(|n| (n.to_owned(), caps.name(n).unwrap().as_str().to_owned()))
                    .collect();

                variables.extend(named_variables);

                for (_, l10n_label) in label.iter_mut() {
                    *l10n_label = tengine::simple_template_render(&l10n_label, &variables)?;
                }

                final_uri = tengine::simple_template_render(&final_uri, &variables)?;
            }

            Ok(Link {
                label: label,
                uri: final_uri,
                rule: Some(self.clone()),
                variables: variables,
            })
        }
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
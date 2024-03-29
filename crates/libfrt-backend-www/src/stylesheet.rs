use std::{collections::HashMap, path::Path};

use anyhow::Result;
use regex::Regex;
use serde::Deserialize;

use libfrt::error::{Error, ErrorKind};
use libfrt::utils::fs::get_mtime;

#[derive(Deserialize, Debug)]
struct StylesheetV2Rule {
    output: String,
    input: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct StylesheetV2 {
    stylesheets: Vec<StylesheetV2Rule>,
    #[serde(default)]
    macros: HashMap<String, String>,
}

#[derive(Default, Clone)]
pub struct StylesheetFile {
    pub contents: String,
    pub mtime: u64,
}

#[derive(Default, Clone)]
pub struct Stylesheets {
    pub sheets: HashMap<String, StylesheetFile>,
}

impl Stylesheets {
    pub fn extend(&mut self, b: Stylesheets) {
        self.sheets.extend(b.sheets);
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let mut ret = Self::default();
        let metafile = path.join("stylesheet_v2.yaml");

        if !metafile.exists() {
            libfrt::bail!(
                InvalidArgument,
                "File stylesheet_v2.yaml was not found in '{}'",
                path.display()
            )
        }

        let mut mtime = get_mtime(&metafile)?;

        let ss: StylesheetV2 = serde_yaml::from_str(std::fs::read_to_string(metafile)?.as_str())?;

        for rule in ss.stylesheets.iter() {
            let mut content = String::from("");

            for f in rule.input.iter() {
                let p = path.join(f);
                let imtime = get_mtime(&p)?;

                if imtime > mtime {
                    mtime = imtime;
                }

                content += format!("\n/* {} */\n", f).as_str();
                content += std::fs::read_to_string(p)?.as_str();
            }

            for (mn, mr) in ss.macros.iter() {
                let regex = Regex::new(format!(r"\${}([ :;\)])", mn).as_str())?;
                content = regex
                    .replace_all(content.as_str(), format!("{}$1", mr).as_str())
                    .to_string();
            }

            let f = StylesheetFile {
                contents: content,
                mtime: mtime,
            };

            ret.sheets.insert(rule.output.clone(), f);
        }

        info!("{} stylesheets compiled", ret.sheets.len());

        Ok(ret)
    }
}

use std::{collections::HashMap, path::Path};

use anyhow::Result;

use crate::error::{Error, ErrorKind};
use crate::utils::fs::get_mtime;
use super::{profile::ProfileWWW, stylesheet::Stylesheets};


#[derive(Default)]
pub(in super) struct TeraIconFactory {
    pub icons: HashMap<String, u32>,
}

impl tera::Function for TeraIconFactory {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let scope = match args.get("scope") {
            Some(var) => var.as_str().unwrap_or("misc"),
            None => "misc",
        };

        let name = match args.get("name") {
            Some(var) => match var.as_str() {
                Some(s) => s,
                None => return Err("Argument 'name' should be str".into()),
            },
            None => return Err("Argument 'name' should not be empty".into()),
        };

        let id = match self.icons.get(format!("{}-{}", scope, name).as_str()) {
            Some(id) => id,
            None => match self.icons.get(format!("{}-fallback", scope).as_str()) {
                Some(id) => id,
                None => match self.icons.get("misc-fallback") {
                    Some(id) => id,
                    None => return Err("No global fallback, check icon".into()),
                },
            },
        };

        Ok(format!("<span class=\"icon\" data-icon=\"&#{};\" aria-hidden=\"true\"></span>", id).into())
    }
}

pub(in super) struct TeraRes {
    pub profile: ProfileWWW,
    pub stylesheets: Stylesheets,
}

impl TeraRes {
    fn find_local_file(&self, dir: &Vec<String>, rr: &str, file: &str, output_prefix: &str) -> Result<String> {
        for i in dir.iter().rev() {
            let p = Path::new(i).join(file);
            if p.is_file() {
                let mtime = get_mtime(p)?;

                return Ok(format!("{}/{}{}?hc=uquery&t={}", rr, output_prefix, file, mtime).into());
            }
        }

        Err(Error::new(ErrorKind::NotExist, "File not found").into())
    }

    fn find_stylesheet_file(&self, rr: &str, file: &str) -> Result<String> {
        match self.stylesheets.sheets.get(file) {
            Some(ss) => {
                Ok(format!("{}/{}?hc=uquery&t={}", rr, file, ss.mtime).into())
            },
            None => Err(Error::new(ErrorKind::NotExist, "File not found").into()),
        }
    }
}

impl tera::Function for TeraRes {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let rr = match args.get("rr") {
            Some(var) => var.as_str().unwrap_or(""),
            None => "",
        };

        let scope = match args.get("scope") {
            Some(var) => var.as_str().unwrap_or(""),
            None => "",
        };

        let file = match args.get("path") {
            Some(var) => match var.as_str() {
                Some(s) => s,
                None => return Err("Argument 'path' should be str".into()),
            }
            None => return Err("Argument 'path' should not be empty".into()),
        };

        let rf = match scope {
            "" | "root" => {
                self.find_local_file(&self.profile.path_static_layers, rr, file, "")
            },
            "icons" => {
                self.find_local_file(&self.profile.path_icon, rr, file, "icons/")
            },
            "styles" => {
                self.find_stylesheet_file(rr, file)
            },
            _ => return Err("Argument 'scope' should be str".into()),
        };

        match rf {
            Ok(f) => Ok(f.into()),
            Err(e) => Err(format!("File not found: {}:{}. Err: {}", scope, file, e).into()),
        }
    }
}

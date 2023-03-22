use std::path::Path;

use anyhow::Result;

use crate::BackendWWW;
use libfrt::{i18n::LangId, utils::fs::get_mtime, ContextData};

pub struct RenderContext<'a> {
    pub backend: &'a BackendWWW,

    pub data: &'a ContextData,
    pub lang: LangId,
}

impl RenderContext<'_> {
    pub fn ui(&self, k: impl AsRef<str>) -> &str {
        todo!()
    }

    pub fn ui_raw(&self, k: impl AsRef<str>) -> Option<toml::Value> {
        todo!()
    }

    pub fn icon(&self, scope: impl AsRef<str>, name: impl AsRef<str>) -> Result<String> {
        let scope = scope.as_ref();
        let name = name.as_ref();

        let id = match self
            .backend
            .icons
            .get(format!("{}-{}", scope, name).as_str())
        {
            Some(id) => id,
            None => match self
                .backend
                .icons
                .get(format!("{}-fallback", scope).as_str())
            {
                Some(id) => id,
                None => match self.backend.icons.get("misc-fallback") {
                    Some(id) => id,
                    None => libfrt::bail!(NotExist, "No global fallback, check icon"),
                },
            },
        };

        Ok(format!(
            "<span class=\"icon\" data-icon=\"&#{};\" aria-hidden=\"true\"></span>",
            id
        ))
    }

    fn find_local_file(
        &self,
        dir: &Vec<String>,
        rr: &str,
        file: &str,
        output_prefix: &str,
    ) -> Result<String> {
        for i in dir.iter().rev() {
            let p = Path::new(i).join(file);
            if p.is_file() {
                let mtime = get_mtime(p)?;

                return Ok(
                    format!("{}/{}{}?hc=uquery&t={}", rr, output_prefix, file, mtime).into(),
                );
            }
        }

        libfrt::bail!(
            NotExist,
            "No conditional file for requested file '{}' in prefix '{}'",
            file,
            output_prefix
        );
    }

    pub fn res(
        &self,
        rr: impl AsRef<str>,
        scope: impl AsRef<str>,
        file: impl AsRef<str>,
    ) -> Result<String> {
        let rr = rr.as_ref();
        let scope = scope.as_ref();
        let file = file.as_ref();

        match scope {
            "" | "root" => {
                self.find_local_file(&self.backend.profile.path_static_layers, rr, file, "")
            }
            "icons" => self.find_local_file(&self.backend.profile.path_icon, rr, file, "icons/"),
            "styles" => match self.backend.stylesheets.sheets.get(file) {
                Some(ss) => Ok(format!("{}/{}?hc=uquery&t={}", rr, file, ss.mtime).into()),
                None => libfrt::bail!(
                    NotExist,
                    "No conditional file for requested stylesheet file '{}'",
                    file
                ),
            },
            _ => libfrt::bail!(
                InvalidArgument,
                "'{}' is not a valid scope param for rc.res()",
                scope
            ),
        }
    }
}

use std::collections::HashMap;

use tera::{Tera, Context};
use anyhow::Result;

pub fn simple_template_render(template: &str, context: &HashMap<String, String>) -> Result<String> {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_template("_", template)?;

    Ok(tera.render("_", &Context::from_serialize(&context)?)?)
}
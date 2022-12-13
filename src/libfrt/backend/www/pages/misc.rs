use std::collections::HashMap;

use anyhow::Result;

use super::Page;
use super::PageRenderOutput;
use super::RenderContext;

pub struct PageMisc {
    pages: HashMap<&'static str, (&'static str, &'static str, &'static str, bool)>,
}

impl PageMisc {
    pub fn new() -> Self {
        Self {
            pages: HashMap::from([
                ("index",     ("index.html",     "..", "/index.html",    true)),
                ("languages", ("languages.html", ".",  "languages.html", false)),
                ("404",       ("404.html",       ".",  "404.html",       false)),
            ]),
        }
    }

    fn render_page(
        rcontext: &RenderContext,
        name: &str,
        template_name: &str,
        rr: &str,
        output_fn: &str,
        i18n_support: bool,
    ) -> Result<PageRenderOutput> {
        info!("Rendering: {}", name);

        let mut tera_context = rcontext.make_common_tera_context()?;
        tera_context.insert("rr", rr);
        tera_context.insert("g_active_page", format!("misc:{}", name).as_str());

        let s = rcontext.tera.render(template_name, &tera_context)?;

        if i18n_support {
            Ok(PageRenderOutput::single_page(
                format!("{}{}", rcontext.lang.as_str(), output_fn),
                s,
            ))
        } else {
            Ok(PageRenderOutput::single_page(
                format!("{}", output_fn),
                s,
            ))
        }
    }
}

impl Page for PageMisc {
    fn render(&self, rcontext: &RenderContext) -> Result<PageRenderOutput> {
        let mut ret = PageRenderOutput::default();

        for (name, cfg) in self.pages.iter() {
            ret.extend(Self::render_page(rcontext, name, cfg.0, cfg.1, cfg.2, cfg.3)?);
        }

        Ok(ret)
    }
}

use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};

pub fn markdown2html(content: &str) -> Result<String> {
    let mut options = Options::empty();
    options.insert(
        Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TABLES
            | Options::ENABLE_MATH
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_SMART_PUNCTUATION
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_GFM,
    );
    let parser = Parser::new_ext(content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(html_output)
}

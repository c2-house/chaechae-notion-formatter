use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref TOC_REGEX: Regex = Regex::new(r"(?i)###\s*(목차|table of contents)").unwrap();
    static ref CALLOUT_REGEX: Regex = Regex::new(r"<aside>\n?([\s\S]*?)\n?</aside>").unwrap();
    static ref LINK_REGEX: Regex = Regex::new(r"^(https?://[^\s]+)$").unwrap();
    static ref IMAGE_REGEX: Regex = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
}

pub fn transform_components(content: &str) -> String {
    let content = add_toc_if_missing(content);
    let content = CALLOUT_REGEX.replace_all(&content, |caps: &Captures| {
        format!("<Callout>\n{}\n</Callout>", &caps[1].trim())
    });
    let content = LINK_REGEX.replace_all(&content, "<Link href=\"$1\" />");
    let content = IMAGE_REGEX.replace_all(&content, "<Image alt=\"$1\" src=\"$2\" />");
    content.to_string()
}

fn add_toc_if_missing(content: &str) -> String {
    if TOC_REGEX.is_match(content) {
        content.to_string()
    } else {
        "### 목차\n\n".to_owned() + content
    }
}

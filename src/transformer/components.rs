use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref TOC_REGEX: Regex = Regex::new(r"(?i)###\s*(목차|table of contents)").unwrap();
    static ref CALLOUT_REGEX: Regex = Regex::new(r"<aside>\n?([\s\S]*?)\n?</aside>").unwrap();
    static ref STANDALONE_LINK_REGEX: Regex = Regex::new(r"(?m)^((https?://[^\s]+))$").unwrap();
    static ref MARKDOWN_LINK_REGEX: Regex = Regex::new(r"\[(https?://.*?)\]\((https?://.*?)\)").unwrap();
    static ref IMAGE_REGEX: Regex = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
}

pub fn transform_components(content: &str) -> String {
    let content = add_toc_if_missing(content);

    let content = CALLOUT_REGEX.replace_all(&content, |caps: &Captures| {
        format!("<Callout>\n{}\n</Callout>", &caps[1].trim())
    });

    let content = STANDALONE_LINK_REGEX.replace_all(&content, "<Link href=\"$1\" />");

    let content = MARKDOWN_LINK_REGEX.replace_all(&content, |caps: &Captures| {
        if caps[1] == caps[2] {
            format!("<Link href=\"{}\" />", &caps[1])
        } else {
            caps[0].to_string()
        }
    });

    let content = IMAGE_REGEX.replace_all(&content, |caps: &Captures| {
        let alt = caps.get(1).map_or("", |m| m.as_str());
        let src = caps.get(2).map_or("", |m| m.as_str());
        format!("<Image alt=\"{}\" src=\"{}\" />", alt, src)
    });

    content.to_string()
}

fn add_toc_if_missing(content: &str) -> String {
    if TOC_REGEX.is_match(content) {
        content.to_string()
    } else {
        "### 목차\n\n".to_owned() + content
    }
}

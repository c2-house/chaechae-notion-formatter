use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref H1_REGEX: Regex = Regex::new(r"^#\s+(.*)").unwrap();
    static ref METADATA_REGEX: Regex = Regex::new(r"^(?P<key>[^:\n]+):\s*(?P<value>.+)").unwrap();
}

pub fn extract_frontmatter(text: &str) -> (String, String) {
    let mut frontmatter = String::from("---\n");
    let mut content = String::new();
    let mut in_metadata = false;

    for line in text.lines() {
        if let Some(caps) = H1_REGEX.captures(line) {
            frontmatter.push_str(&format!("title: {}\n", &caps[1]));
            in_metadata = true;
        } else if in_metadata {
            if let Some(caps) = METADATA_REGEX.captures(line) {
                frontmatter.push_str(&format!("{}: {}\n", &caps["key"], &caps["value"]));
            } else {
                in_metadata = false;
                frontmatter.push_str("---\n");
                content.push_str(line);
                content.push('\n');
            }
        } else {
            content.push_str(line);
            content.push('\n');
        }
    }

    if in_metadata {
        frontmatter.push_str("---\n");
    }

    (frontmatter, content)
}

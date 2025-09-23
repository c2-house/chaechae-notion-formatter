pub mod components;
pub mod frontmatter;

pub fn transform_text(text: &str) -> String {
    let (frontmatter, content) = frontmatter::extract_frontmatter(text);
    let content = components::transform_components(&content);

    format!("{}\n{}", frontmatter, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_text() {
        let input = r#"# This is a title
date: 2024-01-01
category: Rust

This is the content.

<aside>
This is a callout.
</aside>

https://example.com

![alt text](image.png)
"#;

        let expected = r#""---
title: This is a title
date: 2024-01-01
category: Rust
---
### 목차

This is the content.

<Callout>
This is a callout.
</Callout>

<Link href="https://example.com" />

<Image alt="alt text" src="image.png" />
"
"#;
        let (frontmatter, content) = frontmatter::extract_frontmatter(input);
        let content = components::transform_components(&content);
        let result = format!("{}\n{}", frontmatter.trim(), content.trim());

        let expected_processed = expected.replace("\"", "");
        assert_eq!(result.trim(), expected_processed.trim());
    }
}

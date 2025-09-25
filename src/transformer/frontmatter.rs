use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;

lazy_static! {
    static ref H1_REGEX: Regex = Regex::new(r"^#\s+(.*)").unwrap();
    static ref METADATA_REGEX: Regex = Regex::new(r"^(?P<key>[^:\n]+):\s*(?P<value>.+)").unwrap();
}

pub fn extract_frontmatter(text: &str) -> (String, String) {
    let mut frontmatter_map = BTreeMap::new();
    let mut lines = text.lines().peekable();

    if lines.peek().map_or(false, |line| line.starts_with("---")) {
        lines.next();
        while let Some(line) = lines.next() {
            if line.starts_with("---") {
                break;
            }
            if let Some(caps) = METADATA_REGEX.captures(line) {
                frontmatter_map.insert(
                    caps["key"].trim().to_string(),
                    caps["value"].trim().to_string(),
                );
            }
        }
    }

    let mut remaining_lines = lines.collect::<Vec<&str>>().into_iter().peekable();

    if !frontmatter_map.contains_key("title") {
        while let Some(line) = remaining_lines.peek() {
            if line.trim().is_empty() {
                remaining_lines.next();
            } else {
                break;
            }
        }
        if let Some(line) = remaining_lines.peek() {
            if let Some(caps) = H1_REGEX.captures(line) {
                frontmatter_map.insert("title".to_string(), caps[1].trim().to_string());
                remaining_lines.next(); // H1 라인 소모
            }
        }
    }

    while let Some(line) = remaining_lines.peek() {
        if line.trim().is_empty() {
            remaining_lines.next();
        } else {
            break;
        }
    }

    while let Some(line) = remaining_lines.peek() {
        if line.trim().is_empty() {
            remaining_lines.next(); // 본문과 메타데이터를 구분하는 빈 줄 소모
            break;
        }
        if let Some(caps) = METADATA_REGEX.captures(line) {
            let key = caps["key"].trim().to_string();
            if !frontmatter_map.contains_key(&key) {
                frontmatter_map.insert(key, caps["value"].trim().to_string());
            }
            remaining_lines.next();
        } else {
            break;
        }
    }

    let final_content_lines: Vec<&str> = remaining_lines.collect();

    let frontmatter = if frontmatter_map.is_empty() {
        String::new()
    } else {
        let mut fm_string = "---\n".to_string();
        fm_string.push_str(
            &frontmatter_map
                .into_iter()
                .map(|(k, v)| {
                    // 값의 양 끝에 있을 수 있는 따옴표를 먼저 제거합니다.
                    let trimmed_v = v.trim_matches('"');

                    // 'tags' 키는 특별히 배열 형식으로 처리합니다.
                    if k == "tags" {
                        if trimmed_v.starts_with('[') && trimmed_v.ends_with(']') {
                            return format!("{}: {}", k, trimmed_v);
                        } else {
                            return format!("{}: [{}]", k, trimmed_v);
                        }
                    }
                    format!("{}: {}", k, trimmed_v)
                })
                .collect::<Vec<_>>()
                .join("\n"),
        );
        fm_string.push_str("\n---\n");
        fm_string
    };

    (frontmatter, final_content_lines.join("\n"))
}

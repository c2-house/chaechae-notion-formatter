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

    // 1. --- 로 시작하는 Frontmatter 블록이 있는지 확인하고 파싱
    if lines.peek().map_or(false, |line| line.starts_with("---")) {
        lines.next(); // --- 소모
        while let Some(line) = lines.next() {
            if line.starts_with("---") {
                break; // Frontmatter 블록 끝
            }
            if let Some(caps) = METADATA_REGEX.captures(line) {
                frontmatter_map.insert(
                    caps["key"].trim().to_string(),
                    caps["value"].trim().to_string(),
                );
            }
        }
    }

    // 2. 남은 라인들을 수집하여 추가 파싱 준비
    let mut remaining_lines = lines.collect::<Vec<&str>>().into_iter().peekable();

    // 3. H1 제목 처리 (Frontmatter에 title이 없는 경우에만)
    if !frontmatter_map.contains_key("title") {
        // H1 앞에 빈 줄이 있을 수 있으므로 건너뜁니다.
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

    // 4. 메타데이터 블록 시작 전 빈 줄 건너뛰기
    while let Some(line) = remaining_lines.peek() {
        if line.trim().is_empty() {
            remaining_lines.next();
        } else {
            break;
        }
    }

    // 5. 추가 메타데이터 파싱 (기존에 없던 키만 추가)
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
            break; // 메타데이터 형식이 아니면 중단
        }
    }

    // 6. 최종 본문 내용 수집
    let final_content_lines: Vec<&str> = remaining_lines.collect();

    // 7. Frontmatter 문자열 생성
    let frontmatter = if frontmatter_map.is_empty() {
        String::new()
    } else {
        let mut fm_string = "---\n".to_string();
        fm_string.push_str(
            &frontmatter_map
                .into_iter()
                .map(|(k, v)| {
                    let trimmed_v = v.trim_matches('"');
                    if k == "tags" {
                        if trimmed_v.starts_with('[') && trimmed_v.ends_with(']') {
                            return format!("{}: {}", k, trimmed_v);
                        } else {
                            return format!("{}: [{}]", k, trimmed_v);
                        }
                    }

                    if (trimmed_v.starts_with('[') && trimmed_v.ends_with(']'))
                        || trimmed_v.parse::<i64>().is_ok()
                        || trimmed_v.parse::<f64>().is_ok()
                    {
                        format!("{}: {}", k, trimmed_v)
                    } else {
                        format!("{}: \"{}\"", k, trimmed_v.replace('"', "\\\""))
                    }
                })
                .collect::<Vec<_>>()
                .join("\n"),
        );
        fm_string.push_str("\n---\n");
        fm_string
    };

    (frontmatter, final_content_lines.join("\n"))
}

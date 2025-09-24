// image_handler.rs

use crate::config::Config;
use crate::error::NotionFormatterError;
use image::{self, DynamicImage};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::path::Path;
use urlencoding::decode;

lazy_static! {
    static ref IMAGE_REGEX: Regex = Regex::new(r#"<Image alt="(.*?)" src="(.*?)" />"#).unwrap();
}

pub fn process_images_and_update_text(
    config: &Config,
    text: &str,
) -> Result<String, NotionFormatterError> {
    let mut image_counter = 1;
    let mut updated_text = text.to_string();

    let target_images_dir = config.images_dir.join(&config.slug);
    if !target_images_dir.exists() {
        fs::create_dir_all(&target_images_dir)?;
    }

    for caps in IMAGE_REGEX.captures_iter(text) {
        let alt = &caps[1];
        let original_src = &caps[2];

        let decoded_src = decode(original_src)?.to_string();

        let source_filename = Path::new(&decoded_src)
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| {
                NotionFormatterError::InvalidPath(format!("Invalid image src: {}", decoded_src))
            })?;

        // 💡 중요: Config에 명시된 소스 이미지 폴더와 파일 이름을 조합
        let source_image_path = config.source_images_dir.join(source_filename);

        if !source_image_path.exists() {
            println!(
                "⚠️ Image not found, skipping: {}",
                source_image_path.display()
            );
            continue;
        }

        let extension = Path::new(source_filename)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("jpg");

        // 새 파일 이름 (예: "01.png")
        let new_filename = format!("{:02}.{}", image_counter, extension);
        let target_image_path = target_images_dir.join(&new_filename);

        let img = image::open(&source_image_path)?;
        let resized_img = resize_image(img, 900);
        resized_img.save(&target_image_path)?;

        // 💡 최종 src 경로 수정 (요구사항 반영)
        // 결과: "/images/blog/test/01.png"
        let new_src = Path::new("/images/blog")
            .join(&config.slug)
            .join(&new_filename)
            .to_str()
            .unwrap()
            .to_string();

        let original_tag = &caps[0];
        // alt 값은 원본 파일 이름이 아닌, 태그에 있던 alt 값을 그대로 사용
        let new_tag = format!("<Image alt=\"{}\" src=\"{}\" />", alt, new_src);
        updated_text = updated_text.replace(original_tag, &new_tag);

        image_counter += 1;
    }

    Ok(updated_text)
}

fn resize_image(img: DynamicImage, width: u32) -> DynamicImage {
    let aspect_ratio = img.height() as f32 / img.width() as f32;
    let height = (width as f32 * aspect_ratio) as u32;
    img.resize_to_fill(width, height, image::imageops::FilterType::Lanczos3)
}

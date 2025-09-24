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

// 반환 타입을 튜플로 변경: (업데이트된 텍스트, 원본 이미지 폴더 이름)
pub fn process_images_and_update_text(
    config: &Config,
    text: &str,
) -> Result<(String, Option<String>), NotionFormatterError> {
    let mut image_counter = 1;
    let mut updated_text = text.to_string();
    let mut source_image_dir_name: Option<String> = None; // 원본 이미지 폴더 이름을 저장할 변수

    let target_images_dir = config.images_dir.join(&config.slug);
    if !target_images_dir.exists() {
        fs::create_dir_all(&target_images_dir).map_err(|e| {
            NotionFormatterError::DirCreateError(format!(
                "Failed to create target image directory {}: {}",
                target_images_dir.display(),
                e
            ))
        })?;
    }

    for caps in IMAGE_REGEX.captures_iter(text) {
        let alt = &caps[1];
        let original_src = &caps[2];

        let decoded_src = decode(original_src)
            .map_err(|e| NotionFormatterError::InvalidPath(e.to_string()))?
            .to_string();

        // 💡 중요: Notion이 만든 원본 폴더 이름을 캡처합니다.
        if source_image_dir_name.is_none() {
            if let Some(parent) = Path::new(&decoded_src).parent().and_then(|p| p.to_str()) {
                source_image_dir_name = Some(parent.to_string());
            }
        }

        let source_image_path = config.source_dir_path.join(&decoded_src);

        if !source_image_path.exists() {
            println!(
                "⚠️ Image not found, skipping: {}",
                source_image_path.display()
            );
            continue;
        }

        // --- (이하 이미지 리사이즈 및 저장 로직은 동일) ---
        let extension = source_image_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("jpg");

        let new_filename = format!("{:02}.{}", image_counter, extension);
        let target_image_path = target_images_dir.join(&new_filename);

        let img = image::open(&source_image_path).map_err(|e| {
            NotionFormatterError::FileReadError(format!(
                "Failed to open image {}: {}",
                source_image_path.display(),
                e
            ))
        })?;

        let resized_img = resize_image(img, 900);
        resized_img.save(&target_image_path).map_err(|e| {
            NotionFormatterError::FileWriteError(format!(
                "Failed to save image {}: {}",
                target_image_path.display(),
                e
            ))
        })?;
        // --- (여기까지 동일) ---

        // 이 부분은 이미 요구사항대로 잘 동작하고 있습니다.
        let new_src = Path::new("/images/blog")
            .join(&config.slug)
            .join(&new_filename)
            .to_str()
            .unwrap()
            .to_string();

        let original_tag = &caps[0];
        let new_tag = format!("<Image alt=\"{}\" src=\"{}\" />", alt, new_src);
        updated_text = updated_text.replace(original_tag, &new_tag);

        image_counter += 1;
    }

    Ok((updated_text, source_image_dir_name))
}

fn resize_image(img: DynamicImage, width: u32) -> DynamicImage {
    let aspect_ratio = img.height() as f32 / img.width() as f32;
    let height = (width as f32 * aspect_ratio) as u32;
    img.resize_to_fill(width, height, image::imageops::FilterType::Lanczos3)
}

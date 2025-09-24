use crate::{config::Config, error::NotionFormatterError, fs_handler, image_handler, transformer};

pub fn run(config: &Config) -> Result<(), NotionFormatterError> {
    // 1. 파일 읽기
    let content = fs_handler::read_file(&config.source_file_path)?;

    // 2. 텍스트 변환
    let transformed_text = transformer::transform_text(&content);

    // 3. 이미지 처리 및 텍스트 업데이트 (반환 값을 튜플로 받음)
    let (final_text, source_image_dir_name) =
        image_handler::process_images_and_update_text(config, &transformed_text)?;

    // 4. 최종 .mdx 파일 쓰기
    let target_mdx_path = config.posts_dir.join(format!("{}.mdx", config.slug));

    if let Some(parent) = target_mdx_path.parent() {
        fs_handler::create_dir_all(parent)?;
    }
    fs_handler::write_file(&target_mdx_path, &final_text)?;

    // 5. 원본 파일 및 디렉토리 삭제 (💡 수정된 부분)
    if let Some(dir_name) = source_image_dir_name {
        let source_image_dir = config.source_dir_path.join(dir_name);
        fs_handler::delete_file_and_dir(&config.source_file_path, &source_image_dir)?;
    } else {
        // 이미지가 없는 경우 원본 파일만 삭제
        fs_handler::delete_file_and_dir(
            &config.source_file_path,
            &config.source_dir_path.join(""),
        )?;
    }

    println!(
        "✅ Successfully formatted {}",
        config.source_file_path.display()
    );
    println!("   -> Created MDX file: {}", target_mdx_path.display());

    Ok(())
}

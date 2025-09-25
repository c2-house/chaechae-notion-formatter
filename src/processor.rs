use crate::{config::Config, error::NotionFormatterError, fs_handler, image_handler, transformer};

pub fn run(config: &Config) -> Result<(), NotionFormatterError> {
    let content = fs_handler::read_file(&config.source_file_path)?;
    let transformed_text = transformer::transform_text(&content);

    let final_text = image_handler::process_images_and_update_text(config, &transformed_text)?;

    let target_mdx_path = config.posts_dir.join(format!("{}.mdx", config.slug));

    if let Some(parent) = target_mdx_path.parent() {
        fs_handler::create_dir_all(parent)?;
    }
    fs_handler::write_file(&target_mdx_path, &final_text)?;

    fs_handler::delete_file_and_dir(&config.source_file_path, &config.source_dir_path)?;

    println!(
        "✅ Successfully formatted {}",
        config.source_file_path.display()
    );
    println!("   -> Created MDX file: {}", target_mdx_path.display());

    Ok(())
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "nf",
    author = "minchae",
    version = "0.1.0",
    about = "Notion으로 작성한 블로그 포스트를 지정된 MDX 형식으로 자동 변환하는 CLI 도구"
)]
pub struct Cli {
    #[arg(required = true)]
    pub file_path: String,

    #[arg(long, default_value = "../posts/")]
    pub posts_dir: String,

    #[arg(long, default_value = "../public/images/blog/")]
    pub images_dir: String,
}

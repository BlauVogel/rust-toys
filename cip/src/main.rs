use std::str::Split;

use reqwest::Error;
use serde_json::Value;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// 所有者/仓库名，例如 rust-lang/mdBook
    #[structopt(short = "r", long)]
    repo: String,
    /// 文件名，可以使用正则
    #[structopt(short = "f", long)]
    file: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Opt::from_args();
    // 解构出参数内容
    let (mut repo, file) = match args {
        Opt { repo, file } => (repo, file),
    };
    // 得到所有者/仓库名
    let mut v: Split<&str> = repo.split("/");
    let owner = v.next().unwrap().to_string();
    repo = v.next().unwrap().to_string();
    // api url
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases?per_page=1",
        owner, repo
    );
    // 获取 release 信息
    let resp = reqwest::Client::builder()
        .user_agent("BlauVogel")
        .build()?
        .get(url.as_str())
        .send()
        .await?
        .json::<Value>()
        .await?;
    // 解构出下载链接
    let download_url = match &resp {
        Value::Array(array) => match array.iter().next().unwrap() {
            Value::Object(map) => match map.get("assets").unwrap() {
                Value::Array(array) => match array.iter().next().unwrap() {
                    Value::Object(map) => match map.get("browser_download_url").unwrap() {
                        Value::String(url) => url,
                        _ => panic!("错误"),
                    },
                    _ => panic!("错误"),
                },
                _ => panic!("错误"),
            },
            _ => panic!("错误"),
        },
        _ => panic!("错误"),
    };
    println!("Download URL: {}", download_url);

    Ok(())
}

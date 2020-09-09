use std::io;
use std::process::{Child, Command};
use std::str::Split;

use regex::Regex;
use reqwest::Error;
use serde_json::Value;
use structopt::StructOpt;

/// 使用 curl 下载
fn download(file_path: &str, url: &str) -> std::io::Result<Child> {
    Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(file_path)
        .arg(url)
        .spawn()
}

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
    // 解构出所有下载链接，并且匹配
    let url_vec = match &resp {
        Value::Array(array) => match array.iter().next().unwrap() {
            Value::Object(map) => match map.get("assets").unwrap() {
                Value::Array(array) => {
                    let re = Regex::new(&file).unwrap();
                    let mut vec = Vec::new();
                    for a in array {
                        if let Value::Object(map) = a {
                            match map.get("browser_download_url").unwrap() {
                                Value::String(url) => {
                                    let file_name = url.split("/").last().unwrap();
                                    if re.is_match(file_name) {
                                        vec.push(url);
                                    }
                                }
                                _ => panic!("错误"),
                            }
                        }
                    }
                    vec
                }
                _ => panic!("错误"),
            },
            _ => panic!("错误"),
        },
        _ => panic!("错误"),
    };
    // 输出所有匹配的链接，让用户选择
    println!("选择一个进行下载，默认为 0");
    let mut i: usize = 0;
    for url in &url_vec {
        println!("{}) {}", i, url);
        i += 1;
    }
    let mut s = String::new();
    println!("请输入数字：");
    loop {
        io::stdin().read_line(&mut s).expect("Failed to read line");
        if s.len() == 1 {
            i = 0;
            break;
        }
        match s.trim().parse() {
            Ok(num) => {
                i = num;
                break;
            }
            Err(_) => {
                s.clear();
                println!("重新输入：")
            }
        };
    }

    let download_url = url_vec[i];
    let file_name = download_url.split("/").last().unwrap();
    println!("Downloading: {}", download_url);
    // 开始下载
    if let Ok(mut child) = download(&file_name, download_url) {
        if let Err(_) = child.wait() {
            // TODO 删除已经下载的文件
            panic!("下载失败！");
        } else {
            println!("下载完成！");
        }
    } else {
        panic!("创建下载进程失败！");
    }

    Ok(())
}

# cip

cip，即 Cargo Install Plus，是对 cargo install 命令的扩展。

## Why?

像 `cargo install mdbook` 经常是下载源码然后编译，很麻烦，故开发这个小工具来直接下载 github release 中的二进制文件。

## Requirements

- Rust (`cargo`, `rustc`)
- curl (for downloading files)

## Build

```bash
$ git clone --depth=1 https://github.com/BlauVogel/rust-toys
$ cd rust-toys
$ cargo build --package cip --bin cip --release
$ ./target/release/cip --help
```

## 用法

```bash
$ cip --help
cip 0.1.0

USAGE:
    cip --file <file> --repo <repo>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>    文件名，可以使用正则
    -r, --repo <repo>    所有者/仓库名，例如 rust-lang/mdBook
```

例子：

```bash
cip -r rust-lang/mdBook -f mdbook.tar.gz
```

## 参考

- [octocrab](https://github.com/XAMPPRocky/octocrab)：一个第三方的 GitHub API 客户端，实现了 action、pull、search 等，可以参考其代码
- [GitHub Releases API](https://docs.github.com/en/rest/reference/repos#releases)

## TODO

- [ ] 可输出详细日志。`-v` `--verbose` 选项
- [ ] 可自定义安装目录，默认为 `CARGO_HOME/bin`。`-o` `--output` 选项
- [ ] 可设置代理。`-p` `--proxy` 选项

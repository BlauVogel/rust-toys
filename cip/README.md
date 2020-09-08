# cip

cip，即 Cargo Install Plus，是对 cargo install 命令的扩展。像 `cargo install mdbook` 经常是下载源码然后编译，很麻烦，故开发这个小工具来直接下载 github release 中的二进制文件。

## 用法

```bash
❯ cip --help
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
cip -r rust-lang/mdBook -f linux
```

## 参考

- [octocrab](https://github.com/XAMPPRocky/octocrab)：一个第三方的 GitHub API 客户端，实现了 action、pull、search 等，可以参考其代码
- [GitHub Releases API](https://docs.github.com/en/rest/reference/repos#releases)

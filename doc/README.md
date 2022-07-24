<!-- EDIT /Users/z/rmw/env_home/doc/README.md -->

[English](#english-readme) | [中文说明](#中文说明)

---

## English Readme

<!-- EDIT /Users/z/rmw/env_home/doc/en/readme.md -->

env_dir : rust project template

When the git tag is vX.X.X, the binary will be automatically released to github release

A performance report will be generated for each commit (see link below)

> Below is readme template

### Use

[→ examples/main.rs](../examples/main.rs)

```rust
use anyhow::Result;
use env_dir::add;

fn main() -> Result<()> {
  dbg!(add(1, 2));
  Ok(())
}
```


### Install

[Download from github](https://github.com/rmw-lib/env_dir/releases) or `cargo install mdi`

### Link

* [benchmark report log](https://rmw-lib.github.io/env_dir/dev/bench/)

---

## 中文说明

<!-- EDIT /Users/z/rmw/env_home/doc/zh/readme.md -->

env_dir : rust 项目模板

当 git tag 为 vX.X.X 的时候，会自动发版二进制文件到 github release

每次提交都会生成性能报告（见下面链接 ）

> 以下为文档模板

### 安装

[点此下载](https://github.com/rmw-lib/env_dir/releases) 或者 `cargo install mdi`

### 使用

[→ examples/main.rs](../examples/main.rs)

```rust
use anyhow::Result;
use env_dir::add;

fn main() -> Result<()> {
  dbg!(add(1, 2));
  Ok(())
}
```


### 链接

* [性能评测日志](https://rmw-lib.github.io/env_dir/dev/bench/)

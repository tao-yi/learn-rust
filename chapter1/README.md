# 运行 Rust 程序之前必须先编译

```shell
$ rustc hello_world.rs
```

## Cargo

Cargo 是 Rust 的构建系统和包管理工具

- 构建代码，下载依赖的库

安装 Rust 的时候会安装 Cargo

- cargo --version

## 使用 cargo 创建项目

```sh
$ cargo new hello_cargo
```

- `src` 源码都放在这个目录下
  - `main.rs`
- `cargo.toml` 类似于 `package.json`, 是配置文件

```sh
# 构建项目，创建可执行文件 target/debug/hello_cargo
$ cargo build

# 编译代码并且执行，会缓存编译的结果
$ cargo run

# 检查代码，确保能通过编译
$ cargo check

# 为发布而构建，会进行优化，会生成代码在 target/release 目录下
$ cargo build --release
```

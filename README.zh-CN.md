# rust 中文文档

[![原项目](https://img.shields.io/badge/原项目-rust-lang--rust-blue?style=flat-square&logo=github)](https://github.com/rust-lang/rust)
[![微信联系](https://img.shields.io/badge/微信-uaycar-brightgreen?style=flat-square&logo=wechat)](#)

> 本文档是 [rust-lang/rust](https://github.com/rust-lang/rust) 官方 README 的中文翻译版本。
> 代部署 / 定制服务 / 技术咨询 请添加微信:**uaycar**

---

## 简介

这是 Rust 官方的主源码仓库,包含:

- **编译器(rustc)**:将 Rust 源码编译为高效的本机代码;
- **标准库**:Rust 自带的核心库与 alloc 等组件;
- **文档**:语言参考、编译器架构文档等。

Rust 官网:https://www.rust-lang.org/

## 为什么选择 Rust?

- **性能(Performance)**:速度快、内存效率高,适用于关键业务服务和嵌入式设备,并且可以方便地与其他语言集成。

- **可靠性(Reliability)**:丰富的类型系统和所有权模型确保内存安全与线程安全,让大量 bug 在编译期就被消灭。

- **生产力(Productivity)**:文档详尽,编译器以出色的错误诊断著称,工具链先进——包括包管理器和构建工具 [Cargo]、自动格式化工具 [rustfmt]、静态检查器 [Clippy],以及编辑器支持 [rust-analyzer]。

- Rust 也因此常年蝉联 Stack Overflow「最受喜爱编程语言」榜首。

## 快速开始

阅读 [《The Book》中的安装章节](https://doc.rust-lang.org/book/ch01-01-installation.html) 开始上手。最简单的方式是使用 rustup 一键安装:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后验证:

```bash
rustc --version
cargo --version
```

创建第一个项目并运行:

```bash
cargo new hello
cd hello
cargo run
```

## 从源码安装

如果你确实想从源码安装(一般不推荐),请参阅原仓库中的 [INSTALL.md](https://github.com/rust-lang/rust/blob/master/INSTALL.md)。

从源码构建的大致流程(需要在类 Unix 环境或 Windows 上具备相应工具链):

```bash
git clone https://github.com/rust-lang/rust.git
cd rust
./x.py check          # 快速检查构建环境
./x.py build          # 完整构建
```

## 获取帮助

访问 https://www.rust-lang.org/community 查看官方聊天平台(如 Discord、Zulip)与论坛(User Forum、Internals Forum)的完整列表,社区非常活跃友好。

## 参与贡献

请先阅读原仓库的 [CONTRIBUTING.md](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md)。

想深入了解编译器的架构以及如何上手贡献代码,请参阅官方的 [rustc 开发指南(rustc-dev-guide)](https://rustc-dev-guide.rust-lang.org/)。

## 许可证

Rust 主要以 **MIT 许可证** 和 **Apache 许可证 2.0 版** 双许可的方式分发,部分代码由各类类 BSD 许可证覆盖。

详见原仓库中的 [LICENSE-APACHE](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)、[LICENSE-MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) 与 [COPYRIGHT](https://github.com/rust-lang/rust/blob/master/COPYRIGHT)。

## 商标

[Rust 基金会](https://rustfoundation.org/) 拥有并保护 Rust 与 Cargo 的商标及 Logo(即「Rust 商标」)。如需使用这些名称或品牌,请阅读 [Rust 语言商标政策](https://rustfoundation.org/policy/rust-trademark-policy/)。

第三方 Logo 可能受第三方版权与商标约束,详见 [Licenses](https://www.rust-lang.org/policies/licenses)。

---

> **免责声明**:本文档仅为官方 README 的中文翻译,方便中文开发者阅读,内容以英文原版为准。
> 完整源代码与最新动态请访问原项目:https://github.com/rust-lang/rust
>
> **代部署 / 定制服务 / 技术咨询 请添加微信:uaycar**
>
> 如果本项目对你有帮助,请给原项目 [rust-lang/rust](https://github.com/rust-lang/rust) 点个 Star!⭐

<div align="center">

# rust 中文翻译版

**[中文版] rust — 让每个人都能构建可靠且高效软件的系统级编程语言**

[![原项目](https://img.shields.io/badge/原项目-rust-lang--rust-blue?style=flat-square&logo=github)](https://github.com/rust-lang/rust)
[![中文文档](https://img.shields.io/badge/中文文档-README.zh--CN.md-orange?style=flat-square)](README.zh-CN.md)
[![GitHub Stars](https://img.shields.io/github/stars/rust-lang/rust?style=flat-square&label=原项目Stars)](https://github.com/rust-lang/rust/stargazers)
[![微信联系](https://img.shields.io/badge/微信-uaycar-brightgreen?style=flat-square&logo=wechat)](#)

</div>

---

> 这是 [rust-lang/rust](https://github.com/rust-lang/rust) 的中文翻译版本。
> 完整源代码请访问原项目:https://github.com/rust-lang/rust

**代部署 / 定制服务 / 技术咨询 请添加微信:uaycar**

---

## 📖 项目简介

Rust 是一门让所有人都能构建可靠、高效软件的系统级编程语言。本仓库是 Rust 的官方主源码仓库,包含编译器(rustc)、标准库以及相关文档。Rust 凭借丰富的类型系统与所有权模型,在编译期即可保证内存安全与线程安全,同时提供零成本抽象的极致性能,连续多年被评为最受喜爱的编程语言。

## ✨ 主要特性

- **高性能**:速度极快且内存利用率高,没有运行时和 GC,可胜任关键服务、嵌入式设备,并能轻松与其他语言集成。
- **可靠性**:丰富的类型系统和所有权模型确保内存安全与线程安全,大量缺陷在编译期就被拦下。
- **生产力**:文档完善,编译器提供顶级的错误诊断信息,配套工具链成熟。
- **Cargo**:内置包管理器与构建工具,依赖管理、测试、发布一站搞定。
- **rustfmt**:官方代码自动格式化工具,统一团队代码风格。
- **Clippy**:官方静态检查器( linter),帮助写出更地道的 Rust 代码。
- **rust-analyzer**:为各大编辑器/IDE 提供强大的补全、跳转与重构支持。

## 📁 文件说明

| 文件 | 说明 |
|:-----|:-----|
| README.md | 本文件(中文简介) |
| README.zh-CN.md | 详细中文文档(完整汉化) |

## 🚀 快速开始

1. 通过官方推荐方式安装 Rust(rustup 会自动装好 rustc、cargo、标准库等全套工具):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 确认安装成功:

```bash
rustc --version
cargo --version
```

3. 新建并运行第一个项目:

```bash
cargo new hello
cd hello
cargo run
```

4. 更新工具链到最新版:

```bash
rustup update
```

5. 想从源码自行编译(不推荐普通用户使用),请参考原仓库的 [INSTALL.md](https://github.com/rust-lang/rust/blob/master/INSTALL.md)。

6. 学习 Rust 建议从官方教程 [The Book](https://doc.rust-lang.org/book/) 开始,遇到问题可到 [Rust 社区](https://www.rust-lang.org/community) 的聊天平台与论坛求助。

完整源代码与最新版本请访问原项目:https://github.com/rust-lang/rust

## 📞 联系方式

**代部署 / 定制服务 / 技术咨询 请添加微信:uaycar**

---

本项目为 [rust-lang/rust](https://github.com/rust-lang/rust) 的中文翻译版本,所有代码版权归原项目作者所有,遵循其原始许可证(MIT / Apache-2.0 双许可)。

**如果觉得有用,请给原项目点个 Star!** ⭐

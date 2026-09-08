> 🌐 本文档由 [rust-lang/rust](https://github.com/rust-lang/rust) 翻译,英文原版见原项目。

# 参与 Rust 贡献

感谢你有兴趣为 Rust 做出贡献!贡献的方式有很多种,每一种我们都心怀感激。

入门的最佳方式是在 Zulip 的
[#new members](https://rust-lang.zulipchat.com/#narrow/stream/122652-new-members)
频道里寻求帮助。下面我们准备了大量文档,教你如何独立上手,但 Zulip 频道才是*提问*的最佳场所。

为编译器或工具链做贡献的文档位于 [Rustc 开发指南][rustc-dev-guide],通常简称
[rustc-dev-guide]。标准库的开发文档位于 [标准库开发者指南][std-dev-guide],通常简称
[std-dev-guide]。

## 修改子树(subtree)与子模块(submodule)

对于子模块,改动必须提交到子模块对应的仓库,而不是主 `rust-lang/rust` 仓库。

对于子树(subtree),如果改动不需要伴随主 `rust-lang/rust` 仓库的变更
(例如不涉及编译器改动的 rustc-dev-guide 文档更新),请优先向子树对应的仓库提交 PR。

## 关于 [rustc-dev-guide]

[rustc-dev-guide] 旨在记录 rustc(Rust 编译器)的工作原理,并帮助新贡献者参与到
rustc 的开发中来。建议在贡献之前先阅读并理解 [rustc-dev-guide]。这份指南介绍了
Rust 生态中的各类机器人、Rust 开发工具、引导构建(bootstrapping)、编译器架构、
源码表示形式等内容。

## LLM 使用政策

我们针对在向 `rust-lang/rust` 贡献时如何使用大语言模型(LLM)制定了政策,
详见 [Forge 上的政策][LLM policy]。
关于如何*善用* LLM、以及如何审查 LLM 生成的 PR,参见 [开发指南][llm-guidance]。

[LLM policy]: https://forge.rust-lang.org/policies/llm-usage.html
[llm-guidance]: https://rustc-dev-guide.rust-lang.org/llm-guidance.html

## [获取帮助](https://rustc-dev-guide.rust-lang.org/getting-started.html#asking-questions)

遇到卡壳时,获取帮助的途径有很多。Rust 有两个平台可供使用:
[internals] 论坛和 [rust-zulip]。推荐在 [rust-zulip] 上提问,但这两个平台都
非常适合寻求帮助,甚至能找到导师!关于如何提问和获取帮助的更多信息,请阅读
[rustc-dev-guide] 中的[提问](https://rustc-dev-guide.rust-lang.org/getting-started.html#asking-questions)章节。

## Bug 报告

是编译器的错误信息让你来到这里的吗?如果你想提交一份 ICE(内部编译器错误)报告,
请参考[这一节][contributing-bug-reports]并[创建一个 issue][issue template]。

[rustc-dev-guide]: https://rustc-dev-guide.rust-lang.org/
[std-dev-guide]: https://std-dev-guide.rust-lang.org/
[contributing-bug-reports]: https://rustc-dev-guide.rust-lang.org/contributing.html#bug-reports
[issue template]: https://github.com/rust-lang/rust/issues/new/choose
[internals]: https://internals.rust-lang.org
[rust-zulip]: https://rust-lang.zulipchat.com

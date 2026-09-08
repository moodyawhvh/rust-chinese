> 🌐 本文档由 [rust-lang/rust](https://github.com/rust-lang/rust) 翻译,英文原版见原项目。

% Rust 文档

<style>
nav {
    display: none;
}
h3 {
    font-size: 1.35rem;
}
h4 {
    font-size: 1.1rem;
}

/* Formatting for docs search bar */
#search-input {
    width: calc(100% - 58px);
}
#search-but {
    cursor: pointer;
}
#search-but, #search-input {
    padding: 4px;
    border: 1px solid #ccc;
    border-radius: 3px;
    outline: none;
    font-size: 0.7em;
    background-color: #fff;
}
#search-but:hover, #search-input:focus {
    border-color: #55a9ff;
}

/* Formatting for external link icon */
svg.external-link {
  display: inline-block;
  position: relative;
  vertical-align: super;
  width: 0.7rem;
  height: 0.7rem;
  padding-left: 2px;
  top: 3px;
}
</style>

这里是 [Rust 项目]文档总览。本页面收录了各种实用的参考资源链接,其中大部分
可以离线阅读(通过 `rustup doc` 打开)。这些资源多以"书"的形式呈现,我们统称
之为"Rust 书架(The Rust Bookshelf)"。有的鸿篇巨制,有的短小精悍。

这些书全部由 Rust 官方组织维护,不过这里也收录了其他非官方的文档资源!

如果你只是想找标准库参考文档,它在这里:
[Rust API 文档](std/index.html)


## 学习 Rust

如果你想学 Rust,这一节就是为你准备的!以下资源都假设你有一定编程经验,
但不要求掌握任何特定语言:

### The Rust Programming Language

昵称亲切地叫作"the book"的[《The Rust Programming Language》](book/index.html),
会从第一性原理出发带你纵览这门语言。学习过程中你会完成几个小项目,读完后
就能扎实地掌握这门语言的用法。

### Rust By Example

如果你不喜欢捧着几百页的书啃,那 [Rust By Example](rust-by-example/index.html)
正合适。RBE 用大量代码、极少文字展示语言特性,还附带练习!

### Rustlings

[Rustlings](https://github.com/rust-lang/rustlings)
会引导你下载并配置 Rust 工具链,然后通过一个交互式工具教你解决一个个
Rust 编码挑战。

### Rust Playground

[Rust Playground](https://play.rust-lang.org) 是尝试和分享小段代码、
试验热门 crate 的好地方。


## 使用 Rust

熟悉了语言之后,这些资源能帮你把它用起来。

### 标准库

Rust 标准库有[详尽的 API 文档](std/index.html),既讲解各种 API 的用法,
也提供完成各类任务的示例代码。示例代码悬停时会出现"Run"按钮,点击即可在
playground 中运行示例。

<div>
  <form action="std/index.html" method="get">
    <input id="search-input" type="search" name="search"
           placeholder="搜索标准库"/>
    <button id="search-but">搜索</button>
  </form>
</div>

### 你自己的文档

在 crate 中工作时,`cargo doc --open` 会为你的项目*以及*它的全部依赖(以各自
正确的版本)生成文档,并在浏览器中打开。加上 `--document-private-items`
参数,还可以显示未标记 `pub` 的条目。

### Rust 版本历史

[发行说明](releases.html)记录了 Rust 工具链与语言的变更历史。

[版本指南(Edition Guide)](edition-guide/index.html)介绍了 Rust 各个版本
(edition)及其差异。最新工具链支持所有历史版本。

### `rustc` 手册

[`rustc` 手册](rustc/index.html)介绍 Rust 编译器 `rustc`。

### Cargo 手册

[Cargo 手册](cargo/index.html)是 Cargo 的使用指南,Cargo 是 Rust 的构建
工具和依赖管理器。

### Rustdoc 手册

[Rustdoc 手册](rustdoc/index.html)介绍我们的文档工具 `rustdoc`。

### Clippy 手册

[Clippy 手册](clippy/index.html)介绍我们的静态分析器 Clippy。

### 错误码扩展列表

Rust 的许多错误带有错误码,你可以让编译器给出扩展诊断(`rustc --explain`)。
如果愿意,也可以在这里阅读:[rustc 错误码](error_codes/index.html)


## 精通 Rust

当你已经相当熟悉这门语言,这些进阶资源会派上用场。

### 参考(The Reference)

[参考手册](reference/index.html)虽不是正式规范,但比 the book 更详细、
更全面。

### 风格指南

[Rust 风格指南](style-guide/index.html)描述了 Rust 代码的标准格式。大多数
开发者用 `cargo fmt` 调用 `rustfmt` 自动格式化代码(结果与该风格指南一致)。

### Rustonomicon

[Rustonomicon](nomicon/index.html) 是 unsafe Rust 黑魔法指南,有时也被
称为"the 'nomicon"。

### Unstable Book

[The Unstable Book](unstable-book/index.html) 收录了不稳定(unstable)
特性的文档。

### `rustc` 开发指南

[`rustc-dev-guide`](https://rustc-dev-guide.rust-lang.org/)
记录了编译器的工作原理以及如何为它做贡献。如果你想从源码构建或修改 Rust
编译器(比如面向非标准目标),它会非常有用。


## 专用领域

在特定领域使用 Rust 时,可以参考为各领域量身定制的资源。

### 嵌入式系统

开发裸机(Bare Metal)或嵌入式 Linux 系统时,[嵌入式工作组]维护的这些资源
可能对你有用。

[Embedded Working Group]: https://github.com/rust-embedded

#### The Embedded Rust Book

[The Embedded Rust Book] 面向熟悉嵌入式开发和 Rust、但尚未用 Rust 做过
嵌入式开发的开发者。

[The Embedded Rust Book]: embedded-book/index.html
[Rust 项目]: https://www.rust-lang.org

<script>
// check if a given link is external
function isExternalLink(url) {
  const tmp = document.createElement('a');
  tmp.href = url;
  return tmp.host !== window.location.host;
}

// Add the `external` class to all <a> tags with external links and append the external link SVG
function updateExternalAnchors() {
  /*
    External link SVG from Font-Awesome
    CC BY-SA 3.0 https://creativecommons.org/licenses/by-sa/3.0
    via Wikimedia Commons
  */
  const svgText = `<svg
     class='external-link'
     xmlns='http://www.w3.org/2000/svg'
     viewBox='0 -256 1850 1850'
     width='100%'
     height='100%'>
       <g transform='matrix(1,0,0,-1,30,1427)'>
         <path d='M 1408,608 V 288 Q 1408,169 1323.5,84.5 1239,0 1120,
           0 H 288 Q 169,0 84.5,84.5 0,169 0,288 v 832 Q 0,1239 84.5,1323.5 169,
           1408 288,1408 h 704 q 14,0 23,-9 9,-9 9,-23 v -64 q 0,-14 -9,-23 -9,
           -9 -23,-9 H 288 q -66,0 -113,-47 -47,-47 -47,-113 V 288 q 0,-66 47,
           -113 47,-47 113,-47 h 832 q 66,0 113,47 47,47 47,113 v 320 q 0,14 9,
           23 9,9 23,9 h 64 q 14,0 23,-9 9,-9 9,-23 z m 384,864 V 960 q 0,
           -26 -19,-45 -19,-19 -45,-19 -26,0 -45,19 L 1507,1091 855,439 q -10,
           -10 -23,-10 -13,0 -23,10 L 695,553 q -10,10 -10,23 0,13 10,23 l 652,
           652 -176,176 q -19,19 -19,45 0,26 19,45 19,19 45,19 h 512 q 26,0 45,
           -19 19,-19 19,-45 z' style='fill:currentColor' />
         </g>
     </svg>`;
  let allAnchors = document.getElementsByTagName("a");

  for (var i = 0; i < allAnchors.length; ++i) {
    let anchor = allAnchors[i];
    if (isExternalLink(anchor.href)) {
      anchor.classList.add("external");
      anchor.innerHTML += svgText;
    }
  }
}

// on page load, update external anchors
document.addEventListener("DOMContentLoaded", updateExternalAnchors);

</script>

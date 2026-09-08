> 🌐 本文档由 [rust-lang/rust](https://github.com/rust-lang/rust) 翻译,英文原版见原项目。

# 从源码安装

**注意:本文档介绍的是如何*从源码*构建 Rust。如果你不清楚自己在做什么,
*不建议*采用这种方式。如果你只是想安装 Rust,请直接查看 [README.md](README.md)。**

Rust 的构建系统使用一个名为 `x.py` 的 Python 脚本来构建编译器,由它负责管理
引导构建(bootstrap)流程。该脚本位于项目根目录。构建配置由名为 `bootstrap.toml`
的文件决定,完整可选项列表见 `bootstrap.example.toml`。

在大多数 Unix 系统上,`x.py` 命令可以按以下格式直接运行:

```sh
./x.py <子命令> [flags]
```

本文档和示例均假设你以这种方式运行 `x.py`。如果你的平台上这种方式不可用,
请参阅 [rustc 开发指南][rustcguidebuild]。

运行 `x.py --help` 或阅读 [rustc 开发指南][rustcguidebuild]可以了解更多关于
`x.py` 的信息。

[gettingstarted]: https://rustc-dev-guide.rust-lang.org/getting-started.html
[rustcguidebuild]: https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html#what-is-xpy

## 依赖项

请确保已安装以下依赖:

* `python` 3 或 2.7
* `git`
* 一个 C 编译器(构建宿主机目标时 `cc` 即可;交叉编译可能需要额外的编译器)
* `curl`(Windows 上不需要)
* 在 Linux 上编译且目标为 Linux 时需要 `pkg-config`
* `libiconv`(基于 Debian 的发行版中已随 glibc 附带)

要构建 Cargo,还需要 OpenSSL(大多数 Unix 发行版上为 `libssl-dev` 或
`openssl-devel`)。

如果从源码构建 LLVM,还需要以下工具:

* `g++`、`clang++` 或 MSVC,版本要求见
  [LLVM 官方文档](https://llvm.org/docs/GettingStarted.html#host-c-toolchain-both-compiler-and-standard-library)
* `ninja`,或 GNU `make` 3.81 及以上版本(推荐 Ninja,Windows 上尤其如此)
* `cmake`,版本要求见 [LLVM 官方文档](https://llvm.org/docs/GettingStarted.html#software)
* 某些 Linux 发行版(如 Fedora 和 Ubuntu)可能需要 `libstdc++-static`

在 tier 1 或 tier 2 且带 host tools 的平台上,你也可以设置
`llvm.download-ci-llvm = true` 来直接下载预构建的 LLVM。否则,你需要自行安装
LLVM 并确保 `llvm-config` 在 PATH 中。更多信息见
[rustc-dev-guide][sysllvm]。

[sysllvm]: https://rustc-dev-guide.rust-lang.org/building/new-target.html#using-pre-built-llvm


## 在类 Unix 系统上构建

### 构建步骤

1. 用 `git` 克隆[源码][source]:

   ```sh
   git clone https://github.com/rust-lang/rust.git
   cd rust
   ```

[source]: https://github.com/rust-lang/rust

2. 配置构建选项:

   如果不确定该用哪种构建配置、需要一个不错的默认值,可以运行交互式的
   `x.py setup` 命令。它会引导你选择配置档案、设置 LSP、配置 Git 钩子等。

   使用 `configure` 脚本可以在单条命令里处理多项配置,适合生成复杂/高级的
   配置文件。例如:

   ```sh
   ./configure --build=aarch64-unknown-linux-gnu \
      --enable-full-tools \
      --enable-profiler \
      --enable-sanitizers \
      --enable-compiler-docs \
      --set target.aarch64-unknown-linux-gnu.linker=clang \
      --set target.aarch64-unknown-linux-gnu.ar=/rustroot/bin/llvm-ar \
      --set target.aarch64-unknown-linux-gnu.ranlib=/rustroot/bin/llvm-ranlib \
      --set llvm.link-shared=true \
      --set llvm.thin-lto=true \
      --set llvm.libzstd=true \
      --set llvm.ninja=false \
      --set rust.debug-assertions=false \
      --set build.allocator=jemalloc \
      --set rust.bootstrap-override-lld=true \
      --set rust.lto=thin \
      --set rust.codegen-units=1
   ```

   如果你打算用 `x.py install` 进行安装,可以把 `DESTDIR` 环境变量设为你的
   自定义目录路径:

   ```bash
   export DESTDIR=<路径>
   ```

   或者在 `[install]` 配置段中把 `prefix` 和 `sysconfdir` 设为自定义目录路径:

   ```sh
   ./configure --set install.prefix=<路径> --set install.sysconfdir=<路径>
   ```

   当 `DESTDIR` 环境变量存在时,`prefix` 和 `sysconfdir` 的值会与 `DESTDIR`
   环境变量中的路径拼接。

3. 构建并安装:

   ```sh
   ./x.py build && ./x.py install
   ```

   完成后,`./x.py install` 会把若干程序放入 `$PREFIX/bin`:`rustc`(Rust
   编译器)和 `rustdoc`(API 文档工具)。默认还会包含 Rust 的包管理器
   [Cargo]。向 `./configure` 传入 `--set build.extended=false` 可以禁用该行为。

[Cargo]: https://github.com/rust-lang/cargo

### Configure 与 Make

本项目提供 configure 脚本和 makefile(后者只是转调 `x.py`)。`./configure`
是以编程方式生成 `bootstrap.toml` 的推荐方式。不推荐使用 `make`(建议直接用
`x.py`),但我们支持它,并尽量避免无谓地破坏它。

```sh
./configure
make && sudo make install
```

`configure` 生成的 `bootstrap.toml` 同样可以用于普通的 `x.py` 调用。

## 在 Windows 上构建

在 Windows 上,建议使用 [winget] 安装依赖,在终端中运行以下命令:

```powershell
winget install -e Python.Python.3
winget install -e Kitware.CMake
winget install -e Git.Git
```

然后编辑系统的 `PATH` 变量,添加:`C:\Program Files\CMake\bin`。编辑方法见
Java 文档中的[修改系统 `PATH` 指南](https://www.java.com/en/download/help/path.html)。

[winget]: https://github.com/microsoft/winget-cli

Windows 上有两种主流 ABI:Visual Studio 使用的原生(MSVC)ABI,以及 GCC 工具链
使用的 GNU ABI。你需要哪个版本的 Rust,主要取决于想与哪些 C/C++ 库互操作。
要与 Visual Studio 生成的软件互操作,请使用 Rust 的 MSVC 构建;要与
MinGW/MSYS2 工具链构建的 GNU 软件互操作,请使用 GNU 构建。

### MinGW

可以使用 [MSYS2][msys2] 在 Windows 上轻松构建 Rust:

[msys2]: https://www.msys2.org/

1. 下载最新的 [MSYS2 安装程序][msys2]并完成安装。

2. 下载并安装 [Git for Windows](https://git-scm.com/download/win)。确保它在
   Windows PATH 中。为了在 MSYS2 内部访问它,请编辑 MSYS2 安装目录下的
   `mingw[32|64].ini` 文件,取消注释 `MSYS2_PATH_TYPE=inherit` 这一行。

   你也可以改用 `pacman` 安装并使用 MSYS2 自带的 git,但不推荐:它非常慢,
   而且兼容性不常被测试。

3. 从开始菜单启动 MINGW64 或 MINGW32 shell(取决于你要构建 32 位还是 64 位
   Rust),或者从 MSYS2 安装目录(如 `C:\msys64`)运行 `mingw64.exe` 或
   `mingw32.exe`。

4. 在该终端中安装所需工具:

   ```sh
   # 更新包镜像(全新安装 MSYS2 后可能需要)
   pacman -Sy pacman-mirrors

   # 安装 Rust 需要的构建工具。若构建 32 位编译器,
   # 把下面的 "x86_64" 替换为 "i686"。
   # 注意:**不要**使用 'msys2' 子系统的 'python2'、'cmake' 和 'ninja' 包,
   # 使用这些包构建历来容易失败。
   pacman -S make \
               diffutils \
               tar \
               mingw-w64-x86_64-python \
               mingw-w64-x86_64-cmake \
               mingw-w64-x86_64-gcc \
               mingw-w64-x86_64-ninja
   ```

5. 进入 Rust 源码目录(或先克隆),然后构建:

   ```sh
   python x.py setup dist && python x.py build && python x.py install
   ```

   如果你想尝试 Windows 原生版本的 Python 或 CMake,可以从上面的 pacman
   命令中移除它们,改从其他来源安装,并按第 2 步的说明加入 PATH。

   使用 Windows 原生 Python 有助于解决构建 LLVM 时的报错。你也可能想使用
   Git for Windows,因为它通常*快得多*。在 Windows 的"病毒和威胁防护"设置中
   关闭实时保护,也有助于缩短漫长的构建时间(注意它一段时间后会自动重新开启)。

### MSVC

Rust 的 MSVC 构建还需要安装:

- Visual Studio 2022(或更新版本)的构建工具,以便 `rustc` 使用其链接器。
  更旧的版本(如 2019)*也许*能用,但不在主动测试范围内。
- 较新的 Windows 10 或 11 SDK。

最简单的方式是安装 [Visual Studio],勾选"C++ build tools"。

[Visual Studio]: https://visualstudio.microsoft.com/downloads/

(如果你自行安装 CMake,注意不要把"Single components"里的
"C++ CMake tools for Windows" 混进去。)

装好这些依赖后,即可在 `cmd.exe` shell 中构建编译器:

```sh
python x.py setup user
python x.py build
```

目前 Rust 只能用若干已知版本的 Visual Studio 构建。如果你安装了更新的版本而
构建系统无法识别,可能需要强制 bootstrap 使用旧版本。方法是在运行 bootstrap
之前手动调用相应的 vcvars 脚本。

```batch
CALL "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"
python x.py build
```

### 指定 ABI

每种 ABI 也可以在任意环境中通过显式构建三元组(build triple)来使用
(例如在 PowerShell 中使用 GNU ABI)。可用的 Windows 构建三元组:
- GNU ABI(使用 GCC)
    - `i686-pc-windows-gnu`
    - `x86_64-pc-windows-gnu`
- MSVC ABI
    - `i686-pc-windows-msvc`
    - `x86_64-pc-windows-msvc`

构建三元组可以在调用 `x.py` 命令时通过 `--build=<三元组>` 指定,也可以创建
`bootstrap.toml` 文件(如[在类 Unix 系统上构建](#在类-unix-系统上构建)所述)
并向 `./configure` 传入 `--set build.build=<三元组>`。

## 构建文档

如果想构建文档,几乎一样:

```sh
./x.py doc
```

生成的文档会出现在 `build` 目录下对应所用 ABI 的 `doc` 子目录中。也就是说,
如果 ABI 是 `x86_64-pc-windows-msvc`,目录就是
`build\x86_64-pc-windows-msvc\doc`。

## 说明

由于 Rust 编译器本身用 Rust 编写,它必须由一个预编译的"快照"版本
(在更早的开发阶段生成)来构建。因此,源码构建需要联网获取快照,并且操作
系统要能运行这些快照二进制文件。

支持的平台列表见 https://doc.rust-lang.org/nightly/rustc/platform-support.html 。
只有"host tools"平台才有预编译的快照二进制;要为没有 host tools 的平台编译,
必须交叉编译。

其他平台也可能可以工作,但上面列出的才是官方支持的构建环境,成功率最高。

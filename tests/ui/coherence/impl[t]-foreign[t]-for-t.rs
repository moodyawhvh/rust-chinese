//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

extern crate coherence_lib as lib;
use lib::*;
use std::rc::Rc;

struct Local;

impl<T> Remote1<T> for T {
    //~^ ERROR type parameter `T` must be used as an argument to some local type
}

fn main() {}

// ============================================================
// 中文注释(汉化说明,仅注释,不影响测试行为):
// 本测试验证 coherence(一致性/孤儿规则)检查中的 E0210 场景:
// `impl<T> Remote1<T> for T` 试图为本 crate 的泛型参数 T 本身
// 实现外部 crate 的 trait `Remote1<T>`。
//
// 关键点:
// 1. `Remote1` 定义在外部 crate(coherence_lib)中,属于"外部 trait";
// 2. 类型参数 `T` 没有被用作任何"本地类型"的类型实参,
//    而是直接作为 impl 的目标类型(全称 blanket impl);
// 3. 按孤儿规则(E0210),类型参数必须至少出现在某个本地类型
//    的位置上,否则允许外部 crate 的后续改动与此 impl 冲突,
//    因此编译器必须拒绝该 impl,报
//    "type parameter `T` must be used as an argument to some local type"。
// 上方 `//~^ ERROR` 行是 compiletest 的诊断注解,
// 必须与编译器实际输出保持一致,请勿翻译或改动。
// ============================================================

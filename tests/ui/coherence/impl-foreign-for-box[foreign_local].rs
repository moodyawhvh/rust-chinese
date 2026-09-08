//@ compile-flags:--crate-name=test
//@ aux-build:coherence_lib.rs

// Ensure that `Box` in particular isn't fundamental over
// the allocator parameter (but is over T).

#![feature(allocator_api)]

extern crate coherence_lib as lib;

use lib::*;
use std::alloc::{Allocator, AllocError, Layout};
use std::ptr::NonNull;

struct Local;

unsafe impl Allocator for Local {
    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Err(AllocError)
    }
    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
}

impl Remote for Box<str, Local> {}
  //~^ ERROR: only traits defined in the current crate can be implemented for types defined outside of the crate [E0117]

fn main() {}

// ============================================================
// 中文注释(汉化说明,仅注释,不影响测试行为):
// 本测试验证 `Box` 在孤儿规则(fundamental 基本类型规则)下的行为:
// 1. `Box<T, A>` 对类型参数 `T` 而言是 fundamental(基本)类型,
//    因此 `Box<str, Local>` 对 `T = str`(外部类型)的行为等同于 `str` 本身;
// 2. 但 `Box` 对分配器参数 `A` 而言**不是** fundamental:
//    本测试将本地分配器 `Local` 填入 `A` 的位置,构造出
//    `Box<str, Local>`,并尝试为其实现外部 trait `Remote`;
// 3. 由于 `str` 是外部类型、`Local` 虽是本地类型但 `Box` 在该参数上
//    不享受 fundamental 豁免,该 impl 触发 E0117
//    ("only traits defined in the current crate can be implemented
//    for types defined outside of the crate")。
//
// 上方 `//~^ ERROR` 行是 compiletest 的诊断注解,
// 必须与编译器实际输出一致,请勿翻译或改动。
// ============================================================

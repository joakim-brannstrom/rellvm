// Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
// License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
// Author: Joakim Brännström (joakim.brannstrom@gmx.com)

use llvm_sys::core as llvm;
use llvm_sys::prelude::*;

use value::function::Function;

#[derive(Debug)]
pub struct BasicBlock {
    pub ptr: LLVMBasicBlockRef,
}
impl_conv_llvm!(LLVMBasicBlockRef, BasicBlock);

#[derive(Debug)]
pub struct BasicBlockIterator {
    next_: LLVMBasicBlockRef,
}

// TODO the lifetime of BasicBlockIterator should be connected to f. How to do that?
impl<'a> From<&'a Function> for BasicBlockIterator {
    fn from(f: &'a Function) -> Self {
        let first = unsafe { llvm::LLVMGetFirstBasicBlock(f.ptr) };
        BasicBlockIterator { next_: first }
    }
}

impl Iterator for BasicBlockIterator {
    type Item = BasicBlock;

    fn next(&mut self) -> Option<Self::Item> {
        use std::ptr::null_mut;

        if self.next_ == null_mut() {
            return None;
        }

        let cur = self.next_;
        self.next_ = unsafe { llvm::LLVMGetNextBasicBlock(self.next_) };

        Some(BasicBlock::from(cur))
    }
}

#[derive(Debug)]
pub struct EntryBlock {
    pub ptr: LLVMBasicBlockRef,
}
impl_conv_llvm!(LLVMBasicBlockRef, EntryBlock);

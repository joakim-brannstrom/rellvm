// Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
// License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
// Author: Joakim Brännström (joakim.brannstrom@gmx.com)

use llvm_sys::core as llvm;
use llvm_sys::prelude::*;

use value::function::Function;

#[derive(Debug)]
pub struct Parameter {
    pub ptr: LLVMValueRef,
}
impl_conv_llvm!(LLVMValueRef, Parameter);

#[derive(Debug)]
pub struct ParameterIterator {
    next_: LLVMValueRef,
}

// TODO the lifetime of BasicBlockIterator should be connected to f. How to do that?
impl<'a> From<&'a Function> for ParameterIterator {
    fn from(f: &'a Function) -> Self {
        let first = unsafe { llvm::LLVMGetFirstParam(f.ptr) };
        ParameterIterator { next_: first }
    }
}

impl Iterator for ParameterIterator {
    type Item = Parameter;

    fn next(&mut self) -> Option<Self::Item> {
        use std::ptr::null_mut;

        if self.next_ == null_mut() {
            return None;
        }

        let cur = self.next_;
        self.next_ = unsafe { llvm::LLVMGetNextParam(self.next_) };

        Some(Parameter::from(cur))
    }
}

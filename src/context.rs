//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim Brännström (joakim.brannstrom@gmx.com)

use std::ffi::{CString};

use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use module::*;

#[derive(Debug)]
pub struct Context {
    pub ptr: LLVMContextRef,
    // add diagnostics
}
impl_conv_llvm!(LLVMContextRef, Context);

impl Context {
    pub fn new() -> Self {
        Context {
            ptr: unsafe { llvm::LLVMContextCreate() }
        }
    }

    pub fn get_global() -> Self {
        Context {
            ptr: unsafe { llvm::LLVMGetGlobalContext() }
        }
    }

    pub fn make_module(&self, name: &str) -> Module {
        let cname = CString::new(name).unwrap();
        let raw_m = unsafe {
            llvm::LLVMModuleCreateWithNameInContext(cname.as_ptr(), self.ptr)
        };
        Module { ptr: raw_m }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { llvm::LLVMContextDispose(self.ptr); }
    }
}

impl Default for Context {
    fn default() -> Self {
        Context::get_global()
    }
}

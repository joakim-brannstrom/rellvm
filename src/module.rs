//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim BrännströmJoakim Brännström (joakim.brannstrom@gmx.com)
use llvm_sys::prelude::*;

#[derive(Debug)]
pub struct Module {
    pub ptr: LLVMModuleRef
}
impl_conv_llvm!(LLVMModuleRef, Module);

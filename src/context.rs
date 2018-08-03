//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim Brännström (joakim.brannstrom@gmx.com)

use std::ffi::CString;
use std::ptr::null_mut;

use llvm_sys::bit_reader as llvm_bit_reader;
use llvm_sys::core as llvm;
use llvm_sys::prelude::*;

use diagnostic::*;
use memory::*;
use module::*;
use value::metadata::*;

#[derive(Debug)]
pub struct Context {
    pub ptr: LLVMContextRef,
    // add diagnostics

    // add a Vec<Rc<LLVMModuleRef>> so the context can warn if it is being destroyed while there
    // are active modules

    // TODO make this private
    pub ctx: DiagnosticHandlerCtx,
}

impl Context {
    pub fn new() -> Self {
        let mut r = Context {
            ptr: unsafe { llvm::LLVMContextCreate() },
            ctx: DiagnosticHandlerCtx::new(),
        };
        r.register_diagnostic_handler();
        return r;
    }

    // TODO this should be a Rc<Context> to ensure that it is the same instance used everywhere.
    // The severe problem is that each call to get_global will overwrite the diagnostic handler.
    pub fn get_global() -> Self {
        let mut r = Context {
            ptr: unsafe { llvm::LLVMGetGlobalContext() },
            ctx: DiagnosticHandlerCtx::new(),
        };
        r.register_diagnostic_handler();
        return r;
    }

    /// Register a handler that will get callbacks from LLVM and store them for later retrieval.
    fn register_diagnostic_handler(&mut self) {
        ::diagnostic::register(self);
    }

    pub fn new_module(&self, name: &str) -> Module {
        let cname = CString::new(name).unwrap();
        let raw_m = unsafe { llvm::LLVMModuleCreateWithNameInContext(cname.as_ptr(), self.ptr) };
        Module::from(raw_m)
    }

    /// New `Module` from the specified `MemoryBuffer`.
    ///
    /// The ModuleID is derived from `buffer`.
    ///
    /// # Errors
    /// Diagnostic messages that are created by this operation when it fails. This should make it
    /// easier for a human to understand why it failed.
    ///
    /// # Safety
    /// The safety is based on this assumption:
    /// The memory buffer is only needed to create the module. It do not have to be kept alive
    /// after this function has finished.
    pub fn new_module_from_buf(&mut self, buf: &MemoryBuffer) -> Result<Module, Vec<Diagnostic>> {
        let mut module: LLVMModuleRef = null_mut();
        let module_p: *mut LLVMModuleRef = &mut module;

        let success =
            unsafe { llvm_bit_reader::LLVMParseBitcodeInContext2(self.ptr, buf.ptr, module_p) };

        if success == 0 {
            Ok(Module::from(module))
        } else {
            Err(self.ctx.drain())
        }
    }

    /// Resolve a named node extracted from a module.
    ///
    /// # Examples
    /// let m; // a module from somewhere
    /// let resolved = ctx.resolve_named_metadata(m);
    /// let ops = resolved.get_operands();
    ///
    /// # Errors
    /// If for any reason LLVM returned a null pointer.
    ///
    /// # Safety
    /// It should be safe to use this function as long as these two facts hold up:
    /// 1. The user only create `NamedMetadata` instances via a `Module`.
    /// 2. LLVM returns a null pointer if for any reason there is a LLVMValueRef that can't be
    ///    resolved.
    pub fn resolve_named_metadata(&self, nmd: &NamedMetadata) -> Option<ResolvedNamedMetadata> {
        use std::ptr::null_mut;

        let p = nmd.nodes.as_ptr();
        let len = nmd.nodes.len();

        let ref_ = unsafe {
            // p is never modified but the API requires it to be mutable. This is thus safe as long
            // as the C++ code do not misbehave.
            //
            // See Core.hpp
            // Obtain a MDNode value from a context.
            // The returned value corresponds to the llvm::MDNode class.
            llvm::LLVMMDNodeInContext(self.ptr, p as *mut LLVMValueRef, len as u32)
        };

        if ref_ == null_mut() {
            None
        } else {
            Some(ResolvedNamedMetadata::from(ref_))
        }
    }

    /// Obtain a `MetadataString` from a context.
    ///
    /// # Safety
    /// The assumption that how LLVMMDStringInContext is safetly used is built upon the source code
    /// comment. The following is copied from Core.h:
    /// The instance is specified by string data of a specified length. The
    /// string content is copied, so the backing memory can be freed after
    /// this function returns.
    pub fn metadata_string(&self, name: &str) -> Option<MetadataString> {
        use std::ptr::null_mut;

        let p: *const ::libc::c_char = name.as_ptr() as *const ::libc::c_char;
        let len: ::libc::c_uint = name.len() as ::libc::c_uint;

        // Obtain a MDString value from a context.
        //
        // The returned instance corresponds to the llvm::MDString class.
        //
        // The instance is specified by string data of a specified length. The
        // string content is copied, so the backing memory can be freed after
        // this function returns.
        let ref_ = unsafe { llvm::LLVMMDStringInContext(self.ptr, p, len) };

        if ref_ == null_mut() {
            return None;
        } else {
            return Some(MetadataString::from(ref_));
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMContextDispose(self.ptr);
        }
    }
}

//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim BrännströmJoakim Brännström (joakim.brannstrom@gmx.com)

use std::ffi::CStr;
use std::path;

use llvm_sys::core as llvm;
use llvm_sys::prelude::*;

use context::*;
use diagnostic::*;
use memory::*;
use utils::*;

/// Represent a LLVM module that optional owns its backing buffer.
/// The buffer is required if the module is lazily created.
#[derive(Debug)]
pub struct Module {
    pub ptr: LLVMModuleRef,
    pub buf: Option<MemoryBuffer>,
}

impl From<LLVMModuleRef> for Module {
    fn from(ptr: LLVMModuleRef) -> Self {
        Module {
            ptr: ptr,
            buf: None,
        }
    }
}

impl From<Module> for LLVMModuleRef {
    fn from(v: Module) -> Self {
        v.ptr
    }
}

/// Errors that constructing a module from a file can raise.
pub enum ErrModuleFromFile {
    /// Error that can occure when LLVM try to create a MemoryBuffer from a file.
    ReadFile(String),
    /// Messages from LLVM if it failes to create the module in the provided context.
    CreateModule(Vec<Diagnostic>),
}

impl Module {
    pub fn from_file(ctx: &mut Context, file: &path::Path) -> Result<Module, ErrModuleFromFile> {
        let mbuf = match MemoryBuffer::from_file(file) {
            Ok(b) => b,
            Err(msg) => return Err(ErrModuleFromFile::ReadFile(msg.into_owned())),
        };

        let m = match ctx.new_module_from_buf(&mbuf) {
            Ok(m) => m,
            Err(diag) => return Err(ErrModuleFromFile::CreateModule(diag)),
        };
        return Ok(m);
    }

    /// The buffer is required to exist as long as the module do for a lazily created module.
    pub fn from_lazy_ref(ptr: LLVMModuleRef, buf: MemoryBuffer) -> Self {
        Module {
            ptr: ptr,
            buf: Some(buf),
        }
    }

    /// NOTE: the LLVM API do not require [`Context`] to be mutable but in reality it do mutate the
    /// context. I therefor chose to add `mut` to enforce this change of the context.
    pub fn from_name(ctx: &mut Context, id: &str) -> Self {
        use std::ffi::CString;

        let cid = CString::new(id).unwrap();
        let ref_ = unsafe { llvm::LLVMModuleCreateWithNameInContext(cid.as_ptr(), ctx.ptr) };
        Module {
            ptr: ref_,
            buf: None,
        }
    }

    /// Verifies the module.
    ///
    /// # Errors
    /// Returns a human-readable description of any invalid constructs.
    pub fn verify(&self) -> Result<(), Message> {
        use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};
        use std::ptr::null_mut;

        let mut msg: *mut ::libc::c_char = null_mut();

        let status = unsafe {
            let msg_p: &mut *mut ::libc::c_char = &mut msg;
            LLVMVerifyModule(
                self.ptr,
                LLVMVerifierFailureAction::LLVMReturnStatusAction,
                msg_p,
            )
        };

        if status == 0 || msg == null_mut() {
            return Ok(());
        } else {
            return Err(Message::from(msg));
        }
    }

    pub fn identifier(&self) -> String {
        let mut len: ::libc::size_t = 0;
        let len_ptr: *mut usize = &mut len;

        // impl of LLVMGetModuleIdentifier is a call to std::string.c_str() which guarantees a null
        // terminated string
        let cid = unsafe { llvm::LLVMGetModuleIdentifier(self.ptr, len_ptr) };

        unsafe { CStr::from_ptr(cid).to_string_lossy().into_owned() }
    }

    /// Set the identifier of the module to `name`.
    pub fn set_identifier(&self, name: &str) {
        let p = name.as_ptr();
        let len = name.len();

        unsafe {
            llvm::LLVMSetModuleIdentifier(
                self.ptr,
                p as *const ::libc::c_char,
                len as ::libc::size_t,
            );
        }
    }

    // pub fn functions(&self) -> Vec<LLVMFunctionRef> {
    // }

    /// Writes a module to a new memory buffer.
    pub fn as_buffer(&self) -> MemoryBuffer {
        use llvm_sys::bit_writer::*;

        let ref_: LLVMMemoryBufferRef = unsafe { LLVMWriteBitcodeToMemoryBuffer(self.ptr) };
        MemoryBuffer::from(ref_)
    }

    /// Copy the buffer to a pretty printed message.
    pub fn as_message(&self) -> Message {
        let msg_p = unsafe { llvm::LLVMPrintModuleToString(self.ptr) };
        Message::from(msg_p)
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe { llvm::LLVMDisposeModule(self.ptr) }
    }
}

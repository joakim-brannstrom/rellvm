//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim Brännström (joakim.brannstrom@gmx.com)

use std::ffi::CString;
use std::path::*;

use llvm_sys::core as llvm;
use llvm_sys::prelude::*;

use utils::*;

#[derive(Debug)]
pub struct MemoryBuffer {
    // should it be Option<LLVMMemoryBufferRef> for those cases it is not initialized?
    pub ptr: LLVMMemoryBufferRef,
}
impl_conv_llvm!(LLVMMemoryBufferRef, MemoryBuffer);

impl MemoryBuffer {
    pub fn from_file(file: &Path) -> Result<MemoryBuffer, Message> {
        use std::ptr::null_mut;

        let cfile = CString::new(file.to_string_lossy().into_owned()).unwrap();

        let mut mbuf: LLVMMemoryBufferRef = null_mut();
        let mbuf_ptr: *mut LLVMMemoryBufferRef = &mut mbuf;

        let mut msg: *mut ::libc::c_char = null_mut();
        let msg_ptr: *mut *mut ::libc::c_char = &mut msg;

        let status = unsafe {
            llvm::LLVMCreateMemoryBufferWithContentsOfFile(cfile.as_ptr(), mbuf_ptr, msg_ptr)
        };

        match status {
            0 => Ok(MemoryBuffer { ptr: mbuf }),
            _ => Err(Message::from(msg)),
        }
    }

    /// Size of the buffer in bytes.
    pub fn len(&self) -> ::libc::size_t {
        unsafe { llvm::LLVMGetBufferSize(self.ptr) }
    }

    /// Returns a byte slice of this buffers content.
    pub fn as_bytes(&self) -> &[u8] {
        use std::slice::from_raw_parts;

        let begin = unsafe { llvm::LLVMGetBufferStart(self.ptr) as *const u8 };
        unsafe { from_raw_parts(begin, self.len()) }
    }
}

impl Drop for MemoryBuffer {
    fn drop(&mut self) {
        unsafe { llvm::LLVMDisposeMemoryBuffer(self.ptr) }
    }
}

//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim Brännström (joakim.brannstrom@gmx.com)
//!
//! This file contains internal utility functions. They are not ment to be exported to a user of
//! this library user.

use std::ffi::CStr;

use llvm_sys::core as llvm;

/// A LLVM message.
#[derive(Debug)]
pub struct Message {
    pub ptr: *const ::libc::c_char,
}

impl Message {
    pub fn from(ptr: *const ::libc::c_char) -> Self {
        Message { ptr: ptr }
    }

    pub fn into_owned(&self) -> String {
        // TODO should it do a null check?
        unsafe { CStr::from_ptr(self.ptr).to_string_lossy().into_owned() }
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe { llvm::LLVMDisposeMessage(self.ptr as *mut ::libc::c_char) }
    }
}

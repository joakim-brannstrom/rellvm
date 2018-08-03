//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim Brännström (joakim.brannstrom@gmx.com)

use std::vec::*;

use llvm_sys::core as llvm;
use llvm_sys::prelude::*;
use llvm_sys::LLVMDiagnosticSeverity;

use context::*;
use utils::*;

/// Raw diagnostic messages via a handler.
#[derive(Debug)]
pub struct RawDiagnostic {
    pub ptr: LLVMDiagnosticInfoRef,
}
impl_conv_llvm!(LLVMDiagnosticInfoRef, RawDiagnostic);

impl RawDiagnostic {
    //! Extract the severity and human readable text from a `RawDiagnostic`.
    pub fn as_diagnostic(&self) -> Diagnostic {
        let msg = Message::from(unsafe { llvm::LLVMGetDiagInfoDescription(self.ptr) });
        let sev = unsafe { llvm::LLVMGetDiagInfoSeverity(self.ptr) };

        Diagnostic {
            msg: msg,
            severity: sev,
        }
    }
}

/// An extracted diagnostic message from LLVM.
#[derive(Debug)]
pub struct Diagnostic {
    pub msg: Message,
    pub severity: LLVMDiagnosticSeverity,
}

#[derive(Debug)]
pub struct DiagnosticHandlerCtx {
    values: Vec<RawDiagnostic>,
}

impl DiagnosticHandlerCtx {
    pub fn new() -> Self {
        DiagnosticHandlerCtx { values: Vec::new() }
    }

    /// Drain the collected diagnostic messages.
    pub fn drain(&mut self) -> Vec<Diagnostic> {
        self.values
            .drain(..)
            .map(|x| x.as_diagnostic())
            .collect::<Vec<Diagnostic>>()
    }
}

pub fn register(ctx: &mut Context) {
    let ctx_p: *mut ::libc::c_void =
        &mut ctx.ctx as *mut DiagnosticHandlerCtx as *mut ::libc::c_void;
    unsafe { llvm::LLVMContextSetDiagnosticHandler(ctx.ptr, Some(diagnostic_handler), ctx_p) };
}

#[no_mangle]
pub extern "C" fn diagnostic_handler(arg1: LLVMDiagnosticInfoRef, arg2: *mut ::libc::c_void) {
    let ctx_ptr: *mut DiagnosticHandlerCtx = arg2 as *mut DiagnosticHandlerCtx;
    let ctx: &mut DiagnosticHandlerCtx = unsafe { &mut *ctx_ptr };
    ctx.values.push(RawDiagnostic::from(arg1));
}

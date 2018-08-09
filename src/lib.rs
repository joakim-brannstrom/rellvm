//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim Brännström (joakim.brannstrom@gmx.com)
#![allow(dead_code)]

extern crate libc;
extern crate llvm_sys;

#[macro_use]
mod macros;

mod utils;

pub mod ast;
pub mod context;
pub mod diagnostic;
pub mod io;
pub mod memory;
pub mod module;
pub mod value;

// Common traits used by most types

/// A type that has an underlying ptr such as a LLVM*Ref.
/// `macros.rs` contains a helper that implement the trait for a type.
pub trait PtrTrait<T> {
    fn as_ptr(&self) -> &T;
    fn as_mut_ptr(&mut self) -> &mut T;
}

//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim Brännström (joakim.brannstrom@gmx.com)
#![allow(dead_code)]

extern crate libc;
extern crate llvm_sys;

#[macro_use]
mod utils;

mod context;
mod module;

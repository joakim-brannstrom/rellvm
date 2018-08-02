//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim Brännström (joakim.brannstrom@gmx.com)

/// Implement conversion to/from an LLVM wrapped C type.
macro_rules!impl_conv_llvm {
    ($from: ty, $to: tt) => {
        impl From<$from> for $to {
            fn from(ptr: $from) -> Self {
                $to {
                    ptr: ptr,
                }
            }
        }

        impl From<$to> for $from {
            fn from(wrap: $to) -> Self {
                wrap.ptr
            }
        }
    }
}

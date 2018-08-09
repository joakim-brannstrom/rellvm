//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim Brännström (joakim.brannstrom@gmx.com)

use llvm_sys::core as llvm;
use llvm_sys::prelude::*;

use super::*;

/// A node that can count its operands.
///
/// An example in pseudo LLVM IR:
/// !0 = !{i32 4}
/// !1 = !{i32 10}
/// !foo = !{!0, !1}
///
/// foo have two operands.
trait CountOperands {
    fn count_operands(&self) -> u32;
}

/// Represents an individual value in LLVM IR.
///
/// See: llvm-c/Types.h
#[derive(Debug)]
pub struct Raw {
    pub ptr: LLVMValueRef,
}
impl_conv_llvm!(LLVMValueRef, Raw);

/// Represent a node inside a named metadata node.
#[derive(Debug)]
pub struct NamedMetadataNode {
    pub ptr: LLVMValueRef,
}
impl_conv_llvm!(LLVMValueRef, NamedMetadataNode);

/// Represent the operands a named metadata consist of.
///
/// Example of a named metadata node: !foo = !{!4, !3}.
///
/// See `Context` for how to convert this to a MetadataNodeValue.
#[derive(Debug)]
pub struct NamedMetadata {
    pub nodes: Vec<NamedMetadataNode>,
}

/// Correspond to a named metadata node where the operands are resolved.
///
/// An example in pseudo LLVM IR:
/// !0 = !{i32 4}
/// !1 = !{i32 10}
/// !foo = !{!0, !1}
///
/// resolved to:
/// !<anonymouse> = !{!{i32 4}, !{i32 10}}
///
/// Note how it wrappes the data. It is not *true* data because it is still references. One
/// reference, the MDNode, have at least been stripped away.
///
/// This lead to simpler retrival of data via LLVMGetMDNodeOperands.
#[derive(Debug)]
pub struct ResolvedNamedMetadata {
    pub ptr: LLVMValueRef,
}
impl_conv_llvm!(LLVMValueRef, ResolvedNamedMetadata);
impl_ptr_trait!(ResolvedNamedMetadata, LLVMValueRef);

impl ResolvedNamedMetadata {
    /// Obtain the operands.
    pub fn get_operands(&self) -> Option<Vec<Operand>> {
        metadata::get_operands(self)
    }
}

impl CountOperands for ResolvedNamedMetadata {
    fn count_operands(&self) -> u32 {
        unsafe { llvm::LLVMGetMDNodeNumOperands(self.ptr) }
    }
}

/// It is statically known that the operands are all MDNode's wrapping values because this is
/// derived from a named metadata.
///
/// See Core.cpp function LLVMMDStringInContext in the llvm source code.
#[derive(Debug)]
pub struct Operand {
    pub ptr: LLVMValueRef,
}
impl_conv_llvm!(LLVMValueRef, Operand);

impl Operand {
    pub fn as_node(&self) -> Metadata {
        Metadata::from(self.ptr)
    }
}

/// Correspond to an unnamed metadata node.
///
/// Example of a node: !0 = !{!"test\00", i32 10}
///
/// For the composite nodes (DICompositeType) types.
/// See: https://llvm.org/docs/LangRef.html#metadata
#[derive(Debug)]
pub struct Metadata {
    pub ptr: LLVMValueRef,
}
impl_conv_llvm!(LLVMValueRef, Metadata);
impl_ptr_trait!(Metadata, LLVMValueRef);

impl Metadata {
    pub fn get_operands(&self) -> Option<Vec<Operand2>> {
        metadata::get_operands(self)
    }
}

impl CountOperands for Metadata {
    fn count_operands(&self) -> u32 {
        unsafe { llvm::LLVMGetMDNodeNumOperands(self.ptr) }
    }
}

#[derive(Debug)]
pub struct Operand2 {
    pub ptr: LLVMValueRef,
}
impl_conv_llvm!(LLVMValueRef, Operand2);

#[derive(Debug)]
pub struct MetadataString {
    pub ptr: LLVMValueRef,
}
impl_conv_llvm!(LLVMValueRef, MetadataString);

impl MetadataString {
    /// Convert the metadata string reference to a String.
    ///
    /// # Errors
    /// `None` is returned if LLVM ever returns a null ptr.
    ///
    /// # Safety
    /// As long as a MetadataString is not manually created by a user this function should be safe
    /// to use.
    pub fn as_string(&self) -> Option<String> {
        use std::ptr::null;

        let mut len: ::libc::c_uint = 0;
        let len_ptr: *mut ::libc::c_uint = &mut len;

        unsafe {
            // Note: I assume that LLVm still own the string.
            //
            // From Core.h:
            // Obtain the underlying string from a MDString value.
            //
            // @param V Instance to obtain string from.
            // @param Length Memory address which will hold length of returned string.
            // @return String data in MDString.
            let raw: *const ::libc::c_char = llvm::LLVMGetMDString(self.ptr, len_ptr);
            if raw == null() {
                return None;
            }

            let s = std::slice::from_raw_parts(raw as *const u8, len as usize);
            return Some(std::string::String::from_utf8_lossy(&s[..]).into_owned());
        }
    }
}

/// Obtain the given MDNode's operands.
///
/// It is statically known that the operands are all MDNode's wrapping values
/// because this is derived from `NamedMetadata`.
///
/// See Core.cpp function LLVMMDStringInContext in the llvm source code.
fn get_operands<T, U: From<LLVMValueRef>>(node: &T) -> Option<Vec<U>>
where
    T: CountOperands + PtrTrait<LLVMValueRef>,
{
    let ops = node.count_operands();
    if ops == 0 {
        return None;
    }

    let mut refs = Vec::<LLVMValueRef>::with_capacity(ops as usize);
    assert!(
        (ops as usize) <= refs.len(),
        "not enough memory allocated in Vec to hold all operands"
    );

    // TODO probably enough with `p` if from_raw_parts can be skipped
    let p = refs.as_mut_ptr();

    unsafe {
        // from Core.hpp
        //
        // Obtain the given MDNode's operands.
        //
        // The passed LLVMValueRef pointer should point to enough memory to hold all of
        // the operands of the given MDNode (see LLVMGetMDNodeNumOperands) as
        // LLVMValueRefs. This memory will be populated with the LLVMValueRefs of the
        // MDNode's operands.
        //
        // @param V MDNode to get the operands from.
        // @param Dest Destination array for operands.
        llvm::LLVMGetMDNodeOperands(*node.as_ptr(), p);
    }

    Some(refs.into_iter().map(|x| U::from(x)).collect())
}

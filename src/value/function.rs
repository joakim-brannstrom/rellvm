//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim Brännström (joakim.brannstrom@gmx.com)

use llvm_sys::core as llvm;
use llvm_sys::prelude::*;

use value::basic_block::*;
use value::parameter::*;

#[derive(Debug)]
pub struct Function {
    pub ptr: LLVMValueRef,
}
impl_conv_llvm!(LLVMValueRef, Function);

impl Function {
    /// An iterator over the the parameters for the function.
    pub fn parameters(&self) -> Vec<Parameter> {
        let len = unsafe { llvm::LLVMCountParams(self.ptr) };

        let mut params = Vec::new();

        for idx in 0..len {
            let param = unsafe { llvm::LLVMGetParam(self.ptr, idx) };
            params.push(Parameter::from(param));
        }

        return params;
    }

    pub fn count_basic_blocks(&self) -> u32 {
        unsafe { llvm::LLVMCountBasicBlocks(self.ptr) }
    }

    /// Obtain the basic block that corresponds to the entry point of a function.
    ///
    /// # Safety
    /// Assumption: A function always have an entry block thus LLVM would never return a nullptr.
    pub fn entry_block(&self) -> EntryBlock {
        let bb = unsafe { llvm::LLVMGetEntryBasicBlock(self.ptr) };
        EntryBlock::from(bb)
    }

    /// An iterator over the basic blocks in the function.
    pub fn basic_blocks(&self) -> BasicBlockIterator {
        return BasicBlockIterator::from(self);
    }

    /// Check whether the given function has a personality function.
    ///
    /// @see llvm::Function::hasPersonalityFn()
    pub fn has_personality_fn(&self) -> bool {
        unsafe { llvm::LLVMHasPersonalityFn(self.ptr) != 0 }
    }

    /// Obtain the personality function attached to the function.
    ///
    /// @see llvm::Function::getPersonalityFn()
    pub fn personality_fn(&self) -> Option<Function> {
        use std::ptr::*;

        let v = unsafe { llvm::LLVMGetPersonalityFn(self.ptr) };
        if v == null_mut() {
            None
        } else {
            Some(Function::from(v))
        }
    }

    // TODO the following is a list of functions to implement

    // Set the personality function attached to the function.
    //
    // @see llvm::Function::setPersonalityFn()
    //void LLVMSetPersonalityFn(LLVMValueRef Fn, LLVMValueRef PersonalityFn);

    // Obtain the ID number from a function instance.
    //
    // @see llvm::Function::getIntrinsicID()
    // FuncIntrinsicId intrinsicId() {
    //     return LLVMGetIntrinsicID(value).FuncIntrinsicId;
    // }

    // Obtain the calling function of a function.
    //
    // The returned value corresponds to the LLVMCallConv enumeration.
    //
    // @see llvm::Function::getCallingConv()
    // LxCallConv callConv() {
    //     return LLVMGetFunctionCallConv(value).toCallConv;
    // }

    //  Remove a function from its containing module and deletes it.
    //
    // @see llvm::Function::eraseFromParent()
    // void remove() {
    //     LLVMDeleteFunction(this);
    // }

    // Set the calling convention of a function.
    //
    // @see llvm::Function::setCallingConv()
    //
    // @param Fn Function to operate on
    // @param CC LLVMCallConv to set calling convention to
    //void LLVMSetFunctionCallConv(LLVMValueRef Fn, unsigned CC);

    // Obtain the name of the garbage collector to use during code
    // generation.
    //
    // @see llvm::Function::getGC()
    //const char *LLVMGetGC(LLVMValueRef Fn);

    // Define the garbage collector to use during code generation.
    //
    // @see llvm::Function::setGC()
    //void LLVMSetGC(LLVMValueRef Fn, const char *Name);

    // Add an attribute to a function.
    //
    // @see llvm::Function::addAttribute()
    //void LLVMAddAttributeAtIndex(LLVMValueRef F, LLVMAttributeIndex Idx,
    //                             LLVMAttributeRef A);
    //unsigned LLVMGetAttributeCountAtIndex(LLVMValueRef F, LLVMAttributeIndex Idx);
    //void LLVMGetAttributesAtIndex(LLVMValueRef F, LLVMAttributeIndex Idx,
    //                              LLVMAttributeRef *Attrs);
    //LLVMAttributeRef LLVMGetEnumAttributeAtIndex(LLVMValueRef F,
    //                                             LLVMAttributeIndex Idx,
    //                                             unsigned KindID);
    //LLVMAttributeRef LLVMGetStringAttributeAtIndex(LLVMValueRef F,
    //                                               LLVMAttributeIndex Idx,
    //                                               const char *K, unsigned KLen);
    //void LLVMRemoveEnumAttributeAtIndex(LLVMValueRef F, LLVMAttributeIndex Idx,
    //                                    unsigned KindID);
    //void LLVMRemoveStringAttributeAtIndex(LLVMValueRef F, LLVMAttributeIndex Idx,
    //                                      const char *K, unsigned KLen);

    // Add a target-dependent attribute to a function
    // @see llvm::AttrBuilder::addAttribute()
    //void LLVMAddTargetDependentFunctionAttr(LLVMValueRef Fn, const char *A,
    //                                        const char *V);

    // Append a basic block to the end of a function.
    //
    // @see llvm::BasicBlock::Create()
    //LLVMBasicBlockRef LLVMAppendBasicBlockInContext(LLVMContextRef C,
    //                                                LLVMValueRef Fn,
    //                                                const char *Name);

    // Append a basic block to the end of a function using the global
    // context.
    //
    // @see llvm::BasicBlock::Create()
    //LLVMBasicBlockRef LLVMAppendBasicBlock(LLVMValueRef Fn, const char *Name);

    // Insert a basic block in a function before another basic block.
    //
    // The function to add to is determined by the function of the
    // passed basic block.
    //
    // @see llvm::BasicBlock::Create()
    //LLVMBasicBlockRef LLVMInsertBasicBlockInContext(LLVMContextRef C,
    //                                                LLVMBasicBlockRef BB,
    //                                                const char *Name);

    // Insert a basic block in a function using the global context.
    //
    //LLVMBasicBlockRef LLVMInsertBasicBlock(LLVMBasicBlockRef InsertBeforeBB,
    //                                       const char *Name);
}

//! Copyright: Copyright (c) 2018, Joakim Brännström. All rights reserved.
//! License: [Boost Software License 1.0](http://www.boost.org/LICENSE_1_0.txt)
//! Author: Joakim Brännström (joakim.brannstrom@gmx.com)
//!
//!  This file contains convenient functionality for reading/writing LLVM bitcode and IR.

use std::fs::File;
use std::io::prelude::*;

use module::*;

/// Write a LLVM module to the file.
///
/// The function seems silly. It is mostly for self documenting purpose.
///
/// NOTE: The file extension should be ".bc".
/// NOTE: I have confirmed that this function produces the same output as if
/// LLVMWriteBitcodeToFile is called. See the unittests further down.
pub fn write_module(f: &mut File, m: &Module) -> ::std::io::Result<()> {
    f.write_all(m.as_buffer().as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use context::*;
    use std::ffi::CString;
    use std::fs;
    use std::path;

    #[test]
    fn shall_dump_the_module_to_a_file() {
        let mut ctx = Context::new();
        let m = Module::from_name(&mut ctx, "as_file");

        let fname = "remove_me_shall_dump_the_module_to_a_file.bc";
        {
            let mut f = File::create(fname).unwrap();
            assert!(
                write_module(&mut f, &m).is_ok(),
                "something failed when writing to the file"
            );
        }

        assert!(fs::metadata(fname).is_ok(), "module file not written");
        assert!(
            fs::remove_file(fname).is_ok(),
            "unable to cleanup after the test"
        );
    }

    #[test]
    fn shall_be_a_written_module_eq_to_llvm_api_for_writing_modules() {
        use llvm_sys::bit_writer;

        // arrange
        let llvm_io_output = "remove_me_llvm_io_shall_be_a_written_module_eq_to_llvm_api.bc";
        let rellvm_io_output = "remove_me_rellvm_io_shall_be_a_written_module_eq_to_llvm_api.bc";

        let mut ctx = Context::new();
        let test_m = Module::from_name(&mut ctx, "test");

        // act
        {
            let c_llvm_io_output = CString::new(llvm_io_output).unwrap();
            assert!(
                unsafe {
                    bit_writer::LLVMWriteBitcodeToFile(test_m.ptr, c_llvm_io_output.as_ptr())
                } == 0,
                "failed writing module via LLVMs bitwrite API"
            );
        }
        {
            let mut f = File::create(rellvm_io_output).unwrap();
            write_module(&mut f, &test_m).unwrap();
        }

        // assert
        let llvm_io_data = fs::read(llvm_io_output).unwrap();
        let rellvm_io_data = fs::read(rellvm_io_output).unwrap();
        assert_eq!(rellvm_io_data.as_slice(), llvm_io_data.as_slice());

        assert!(fs::remove_file(llvm_io_output).is_ok());
        assert!(fs::remove_file(rellvm_io_output).is_ok());
    }

    #[test]
    fn the_written_file_shall_be_eq_to_the_original_when_read_back() {
        // arrange
        let mut original_ctx = Context::new();
        let original_module = Module::from_name(&mut original_ctx, "some name");
        let module_fname =
            "remove_me_the_written_file_shall_be_eq_to_the_original_when_read_back.bc";

        // act
        {
            let mut f = File::create(module_fname).unwrap();
            write_module(&mut f, &original_module).unwrap();
        }

        // assert
        let mut read_ctx = Context::new();

        match Module::from_file(&mut read_ctx, path::Path::new(module_fname)) {
            Ok(read_module) => {
                assert_eq!(module_fname, read_module.identifier());
                // adjust the identifier so it is possible to compare the content without the ID
                // leading to a mismatch
                read_module.set_identifier("some name");
                assert_eq!(
                    original_module.as_message().into_owned(),
                    read_module.as_message().into_owned()
                );
            }
            Err(ErrModuleFromFile::ReadFile(msg)) => assert!(false, msg),
            // TODO pretty print the msgs
            Err(ErrModuleFromFile::CreateModule(_msg)) => assert!(false),
        }

        assert!(fs::remove_file(module_fname).is_ok());
    }
}

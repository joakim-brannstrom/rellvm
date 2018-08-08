# rellvm [![Build Status](https://travis-ci.org/joakim-brannstrom/rellvm.svg?branch=master)](https://travis-ci.org/joakim-brannstrom/rellvm)

**rellvm** is a high level wrapper for llvm-sys. Its primary purpose (for now) is to analyze LLVM IR.

# Getting Started

rellvm depends on the following software packages:

 * [Rust compiler](https://www.rust-lang.org/en-US/install.html) (rustc 1.26+)

For users running Ubuntu one of the dependencies can be installed with apt.
```sh
sudo apt install llvm llvm-dev
```

Install the rust compiler:
```sh
curl https://sh.rustup.rs -sSf | sh
```

Once the dependencies are installed it is time to download the source code to install rellvm.
```sh
git clone https://github.com/joakim-brannstrom/rellvm.git
cd rellvm
cargo build --release
```

Done! Have fun.
Don't be shy to report any issue that you find.

# Credit

The developers of the llvm and llvm-sys crate. Awesome work. This crate would not have been possible without those people.

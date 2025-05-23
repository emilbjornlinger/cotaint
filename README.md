# A Cocoon-like static taint-checker for Rust

This repository implements a rudimentary taint-checker for Rust. It is used as a library directly within the Rust compiler toolchain and doesn't involve any third-party tools. The library is heavily based on the Cocoon library (and contains a lot of copied code from there, but has not been made a fork) https://github.com/PLaSSticity/Cocoon-implementation, and implements a subset of the IFC-features presented in the paper https://arxiv.org/pdf/2311.00097. This repository is a proof of concept and can't be used as a tool since some IFC features are missing, meaning that the analysis is not complete.

## Installation

In order to test the functionality of this project, you must download Rust. Instructions how this is done can be found on Rust's website: 

https://www.rust-lang.org/tools/install

Also, the code is not built on stable Rust and therefore requires to be run/compiled in Nightly mode. What it is and how to activate it can be found here:

https://doc.rust-lang.org/book/appendix-07-nightly-rust.html 

## Examples and testing

Examples that tests the functionality of the cotaint library can be found in the cotaint/src/bin folder. Here, different code blocks are tested to ensure that the taint_block() macro can handle different types of code blocks. In the different example files there are examples illustrating compiling and non-compiling code snippets.

Note, not all in-built functions in Rust are supported by the library. The supported functions can be found in cotaint/macros/src/lib.rs and is listed in the function: is_call_to_allowed_function(). 

## Usage

How to build and compile the package:
```
cargo build
```

How to run the tests:
```
cargo run --bin {filename}.rs 
```

where {filename} is the name of the file that you want to run. 

## Project Report

For more information about the library and the project, feel free to read our project report which can be accessed in cotaint/report.pdf

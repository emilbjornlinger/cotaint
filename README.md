# A Cocoon-like static taint-checker for Rust

This repository implements a rudimentary taint-checker for Rust. It is used as a library directly within the Rust compiler toolchain and doesn't involve any third-party tools. The library is heavily based on the Cocoon library https://github.com/PLaSSticity/Cocoon-implementation, and implements a subset of the IFC-features presented in the paper https://arxiv.org/pdf/2311.00097. This repository is a proof of concept and can't be used as a tool since some IFC features are missing, meaning that the analysis is not complete.

# Unsafe Rust Pointer Manipulation Leading to Vector Corruption

This repository demonstrates a potential issue with using raw pointers to manipulate vectors in Rust.  While raw pointers offer fine-grained control, they can easily lead to memory corruption and undefined behavior if not used with extreme caution. 

The `bug.rs` file shows an example where a raw pointer is used to modify the vector's first element. This approach bypasses Rust's memory safety guarantees, potentially leading to data corruption or crashes.

The `bugSolution.rs` file demonstrates a safer approach that utilizes Rust's built-in mechanisms for vector manipulation, avoiding the use of raw pointers and ensuring memory safety.

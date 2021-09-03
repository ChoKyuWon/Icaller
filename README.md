# Icaller - find LLVM-CFI forward edge gadget

## Usage
``cargo run <BINARY_PATH>``

## Plan
1. Find all indirect call and branch(e.g call rax, jmp rcx) 
2. Find all CFI instrument code based on 1.
3. Find all Jump table and allowed offset.
4. Get address form jump table and print it

References:
* [Control Flow Integrity](https://clang.llvm.org/docs/ControlFlowIntegrity.html)
* [Control Flow Integrity Design Documentation](https://clang.llvm.org/docs/ControlFlowIntegrityDesign.html)
* [Enforcing Forward-Edge Control-Flow Integrity in GCC & LLVM](https://static.googleusercontent.com/media/research.google.com/ko//pubs/archive/42808.pdf)
* [Type Metadata](https://llvm.org/docs/TypeMetadata.html)
* [roper](https://github.com/oblivia-simplex/roper)


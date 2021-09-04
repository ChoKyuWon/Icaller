# Icaller - find LLVM-CFI forward edge gadget

## Usage
``cargo run <BINARY_PATH>``

## Plan
- [x] Find all indirect call and branch(e.g call rax, jmp rcx) 
- [ ] Find all CFI instrument code based on 1.
- [ ] Find all Jump table and allowed offset.
- [ ] Get address form jump table and print it

References:
* [Control Flow Integrity](https://clang.llvm.org/docs/ControlFlowIntegrity.html)
* [Control Flow Integrity Design Documentation](https://clang.llvm.org/docs/ControlFlowIntegrityDesign.html)
* [Enforcing Forward-Edge Control-Flow Integrity in GCC & LLVM](https://static.googleusercontent.com/media/research.google.com/ko//pubs/archive/42808.pdf)
* [Type Metadata](https://llvm.org/docs/TypeMetadata.html)
* [roper](https://github.com/oblivia-simplex/roper)


mod chunk;
mod compiler;
mod intern;
mod nanbox;
mod opcode;
mod peephole;
mod vm_nanbox;
pub use chunk::Chunk;
pub use compiler::Compiler;
pub use intern::StringInterner;
pub use nanbox::{check_leaks, heap_stats, reset_stats};
pub use nanbox::{CompiledFunction, HeapData, HeapObject, NanBoxed, ObjectTag};
pub use opcode::OpCode;
pub use peephole::optimize as peephole_optimize;
pub use vm_nanbox::VMNanBox;
pub use vm_nanbox::VMNanBox as VM;


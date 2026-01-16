//! Bytecode VM module for SpecterScript
//!
//! Stack-based virtual machine with NaN-boxed values.
//! v0.9: Single optimized VM implementation.

mod opcode;
mod chunk;
mod compiler;
mod nanbox;
mod vm_nanbox;

pub use opcode::OpCode;
pub use chunk::Chunk;
pub use compiler::Compiler;
pub use nanbox::{NanBoxed, HeapObject, HeapData, ObjectTag, CompiledFunction};
pub use nanbox::{heap_stats, check_leaks, reset_stats};
pub use vm_nanbox::VMNanBox;

// Re-export VMNanBox as VM for clean API
pub use vm_nanbox::VMNanBox as VM;

//! NaN-boxed value representation for high-performance execution
//!
//! Uses IEEE 754 NaN space to pack all values into 64 bits:
//! - Floats: stored directly as f64
//! - Integers: 48-bit signed int in NaN payload
//! - Pointers: 48-bit pointer in NaN payload (heap objects)
//! - Booleans/Nil: special NaN patterns
//!
//! This eliminates discriminant checks and heap allocation for primitives.

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

// === Debug-only allocation tracking ===
// Tracks heap allocations to detect memory leaks in debug builds

#[cfg(debug_assertions)]
static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

#[cfg(debug_assertions)]
static DEALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Get current allocation stats (debug only)
#[cfg(debug_assertions)]
pub fn heap_stats() -> (usize, usize) {
    (
        ALLOC_COUNT.load(Ordering::Relaxed),
        DEALLOC_COUNT.load(Ordering::Relaxed),
    )
}

/// Check for memory leaks - returns number of leaked objects
#[cfg(debug_assertions)]
pub fn check_leaks() -> usize {
    let (alloc, dealloc) = heap_stats();
    alloc.saturating_sub(dealloc)
}

/// Reset allocation counters (for testing)
#[cfg(debug_assertions)]
pub fn reset_stats() {
    ALLOC_COUNT.store(0, Ordering::Relaxed);
    DEALLOC_COUNT.store(0, Ordering::Relaxed);
}

#[cfg(debug_assertions)]
fn track_alloc() {
    ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
}

#[cfg(debug_assertions)]
fn track_dealloc() {
    DEALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
}

// Release builds: no-ops
#[cfg(not(debug_assertions))]
pub fn heap_stats() -> (usize, usize) { (0, 0) }
#[cfg(not(debug_assertions))]
pub fn check_leaks() -> usize { 0 }
#[cfg(not(debug_assertions))]
pub fn reset_stats() {}
#[cfg(not(debug_assertions))]
fn track_alloc() {}
#[cfg(not(debug_assertions))]
fn track_dealloc() {}

// === NaN-boxing bit patterns ===
// IEEE 754 quiet NaN: exponent all 1s, mantissa bit 51 = 1
// We use bits 48-50 for type tags, bits 0-47 for payload

/// Quiet NaN base pattern (bits 51-63)
const QNAN: u64 = 0x7FFC_0000_0000_0000;

/// Tag: nil value
const TAG_NIL: u64 = 0x0001_0000_0000_0000;

/// Tag: boolean false
const TAG_FALSE: u64 = 0x0002_0000_0000_0000;

/// Tag: boolean true  
const TAG_TRUE: u64 = 0x0003_0000_0000_0000;

/// Tag: 48-bit signed integer
const TAG_INT: u64 = 0x0004_0000_0000_0000;

/// Tag: heap pointer (String, List, Map, Function, etc.)
const TAG_PTR: u64 = 0x0005_0000_0000_0000;

/// Singleton patterns
const NIL: u64 = QNAN | TAG_NIL;
const FALSE: u64 = QNAN | TAG_FALSE;
const TRUE: u64 = QNAN | TAG_TRUE;

/// Mask for 48-bit payload
const PAYLOAD_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;

/// Mask to check if value is a quiet NaN (not a regular float)
const QNAN_CHECK: u64 = 0x7FFC_0000_0000_0000;

/// NaN-boxed value - all runtime values in 64 bits
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct NanBoxed(u64);

impl NanBoxed {
    // === Constructors ===
    
    /// Create a number (f64) value
    #[inline(always)]
    pub const fn number(n: f64) -> Self {
        Self(n.to_bits())
    }
    
    /// Create an integer value (48-bit signed)
    #[inline(always)]
    pub const fn integer(n: i64) -> Self {
        // Truncate to 48 bits and add tag
        let bits = (n as u64) & PAYLOAD_MASK;
        Self(QNAN | TAG_INT | bits)
    }
    
    /// Create nil value
    #[inline(always)]
    pub const fn nil() -> Self {
        Self(NIL)
    }
    
    /// Create boolean value
    #[inline(always)]
    pub const fn boolean(b: bool) -> Self {
        Self(if b { TRUE } else { FALSE })
    }
    
    /// Create a heap pointer value
    #[inline(always)]
    pub fn ptr(p: *mut HeapObject) -> Self {
        let addr = p as u64;
        debug_assert!(addr & !PAYLOAD_MASK == 0, "pointer too large for NaN-boxing");
        Self(QNAN | TAG_PTR | addr)
    }
    
    // === Type checks (branchless where possible) ===
    
    /// Check if this is a number (not a NaN-boxed tagged value)
    #[inline(always)]
    pub fn is_number(self) -> bool {
        // If the QNAN bits aren't all set, it's a regular float
        (self.0 & QNAN_CHECK) != QNAN_CHECK
    }
    
    /// Check if this is an integer
    #[inline(always)]
    pub fn is_integer(self) -> bool {
        (self.0 & (QNAN | TAG_INT)) == (QNAN | TAG_INT) && 
        (self.0 & TAG_PTR) != TAG_PTR  // Exclude pointers
    }
    
    /// Check if this is nil
    #[inline(always)]
    pub fn is_nil(self) -> bool {
        self.0 == NIL
    }
    
    /// Check if this is a boolean
    #[inline(always)]
    pub fn is_bool(self) -> bool {
        self.0 == TRUE || self.0 == FALSE
    }
    
    /// Check if this is a heap pointer
    #[inline(always)]
    pub fn is_ptr(self) -> bool {
        (self.0 & (QNAN | TAG_PTR)) == (QNAN | TAG_PTR)
    }
    
    // === Extractors ===
    
    /// Get as f64 number (assumes is_number() is true)
    #[inline(always)]
    pub fn as_number(self) -> f64 {
        f64::from_bits(self.0)
    }
    
    /// Get as i64 integer (assumes is_integer() is true)
    #[inline(always)]
    pub fn as_integer(self) -> i64 {
        // Extract 48-bit payload and sign-extend
        let raw = (self.0 & PAYLOAD_MASK) as i64;
        // Sign-extend from 48 bits
        (raw << 16) >> 16
    }
    
    /// Get as boolean (assumes is_bool() is true)
    #[inline(always)]
    pub fn as_bool(self) -> bool {
        self.0 == TRUE
    }
    
    /// Get as heap pointer (assumes is_ptr() is true)
    #[inline(always)]
    pub fn as_ptr(self) -> *mut HeapObject {
        (self.0 & PAYLOAD_MASK) as *mut HeapObject
    }
    
    /// Get as any numeric value (int or float)
    #[inline(always)]
    pub fn as_numeric(self) -> Option<f64> {
        if self.is_number() {
            Some(self.as_number())
        } else if self.is_integer() {
            Some(self.as_integer() as f64)
        } else {
            None
        }
    }
    
    // === Truthiness ===
    
    /// Check if value is truthy
    #[inline(always)]
    pub fn is_truthy(self) -> bool {
        match self.0 {
            NIL | FALSE => false,
            TRUE => true,
            _ if self.is_number() => self.as_number() != 0.0,
            _ if self.is_integer() => self.as_integer() != 0,
            _ => true, // Strings, lists, etc. are truthy
        }
    }
    
    /// Get raw bits (for debugging)
    #[inline(always)]
    pub fn bits(self) -> u64 {
        self.0
    }
}

impl Default for NanBoxed {
    fn default() -> Self {
        Self::nil()
    }
}

impl fmt::Debug for NanBoxed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_nil() {
            write!(f, "nil")
        } else if self.is_bool() {
            write!(f, "{}", if self.as_bool() { "yes" } else { "no" })
        } else if self.is_number() {
            write!(f, "{}", self.as_number())
        } else if self.is_integer() {
            write!(f, "{}", self.as_integer())
        } else if self.is_ptr() {
            write!(f, "<ptr {:p}>", self.as_ptr())
        } else {
            write!(f, "<unknown 0x{:016X}>", self.0)
        }
    }
}

impl fmt::Display for NanBoxed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_nil() {
            write!(f, "nil")
        } else if self.is_bool() {
            write!(f, "{}", if self.as_bool() { "yes" } else { "no" })
        } else if self.is_number() {
            let n = self.as_number();
            if n.fract() == 0.0 && n.abs() < i64::MAX as f64 {
                write!(f, "{}", n as i64)
            } else {
                write!(f, "{}", n)
            }
        } else if self.is_integer() {
            write!(f, "{}", self.as_integer())
        } else if self.is_ptr() {
            // Delegate to HeapObject's display
            let obj = unsafe { &*self.as_ptr() };
            write!(f, "{}", obj)
        } else {
            write!(f, "<unknown>")
        }
    }
}

// === Heap-allocated objects ===

/// Object type tag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ObjectTag {
    String = 0,
    List = 1,
    Map = 2,
    Function = 3,
    Closure = 4,
    Native = 5,
    Struct = 6,
}

/// Header for all heap-allocated objects
#[repr(C)]
pub struct HeapObject {
    /// Object type
    pub tag: ObjectTag,
    /// Reference count
    pub rc: std::sync::atomic::AtomicU32,
    /// Object data follows inline
    pub data: HeapData,
}

/// Heap object data variants
pub enum HeapData {
    String(Box<str>),
    List(Vec<NanBoxed>),
    Map(std::collections::HashMap<Box<str>, NanBoxed>),
    Function(CompiledFunction),
}

/// A compiled function for the VM
#[derive(Debug, Clone)]
pub struct CompiledFunction {
    /// Function name
    pub name: Box<str>,
    /// Number of parameters
    pub arity: u8,
    /// Number of local variables (including params)
    pub local_count: u8,
    /// Compiled bytecode
    pub chunk: super::Chunk,
}

impl fmt::Display for HeapObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            HeapData::String(s) => write!(f, "{}", s),
            HeapData::List(items) => {
                write!(f, "lst(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            HeapData::Map(map) => {
                write!(f, "map(")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, ")")
            }
            HeapData::Function(func) => write!(f, "<fn {}>", func.name),
        }
    }
}

impl HeapObject {
    /// Allocate a new string on the heap
    pub fn new_string(s: &str) -> *mut Self {
        track_alloc();
        let obj = Box::new(HeapObject {
            tag: ObjectTag::String,
            rc: std::sync::atomic::AtomicU32::new(1),
            data: HeapData::String(s.into()),
        });
        Box::into_raw(obj)
    }
    
    /// Allocate a new list on the heap
    pub fn new_list(items: Vec<NanBoxed>) -> *mut Self {
        track_alloc();
        let obj = Box::new(HeapObject {
            tag: ObjectTag::List,
            rc: std::sync::atomic::AtomicU32::new(1),
            data: HeapData::List(items),
        });
        Box::into_raw(obj)
    }
    
    /// Allocate a new function on the heap
    pub fn new_function(func: CompiledFunction) -> *mut Self {
        track_alloc();
        let obj = Box::new(HeapObject {
            tag: ObjectTag::Function,
            rc: std::sync::atomic::AtomicU32::new(1),
            data: HeapData::Function(func),
        });
        Box::into_raw(obj)
    }
    
    /// Free a heap object
    /// SAFETY: ptr must be a valid pointer returned by new_string/new_list
    pub unsafe fn free(ptr: *mut Self) {
        if !ptr.is_null() {
            track_dealloc();
            drop(Box::from_raw(ptr));
        }
    }
    
    /// Increment reference count
    #[inline]
    pub fn incref(&self) {
        self.rc.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    
    /// Decrement reference count, returns true if object should be freed
    #[inline]
    pub fn decref(&self) -> bool {
        self.rc.fetch_sub(1, std::sync::atomic::Ordering::Release) == 1
    }
}

// === Convenience conversions ===

impl From<f64> for NanBoxed {
    fn from(n: f64) -> Self {
        Self::number(n)
    }
}

impl From<i64> for NanBoxed {
    fn from(n: i64) -> Self {
        Self::integer(n)
    }
}

impl From<i32> for NanBoxed {
    fn from(n: i32) -> Self {
        Self::integer(n as i64)
    }
}

impl From<bool> for NanBoxed {
    fn from(b: bool) -> Self {
        Self::boolean(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_nil() {
        let v = NanBoxed::nil();
        assert!(v.is_nil());
        assert!(!v.is_truthy());
    }
    
    #[test]
    fn test_booleans() {
        let t = NanBoxed::boolean(true);
        let f = NanBoxed::boolean(false);
        
        assert!(t.is_bool());
        assert!(f.is_bool());
        assert!(t.as_bool());
        assert!(!f.as_bool());
        assert!(t.is_truthy());
        assert!(!f.is_truthy());
    }
    
    #[test]
    fn test_numbers() {
        let pi = NanBoxed::number(3.14159);
        assert!(pi.is_number());
        assert!((pi.as_number() - 3.14159).abs() < 1e-10);
        
        let zero = NanBoxed::number(0.0);
        assert!(!zero.is_truthy());
        
        let one = NanBoxed::number(1.0);
        assert!(one.is_truthy());
    }
    
    #[test]
    fn test_integers() {
        let i = NanBoxed::integer(42);
        assert!(i.is_integer());
        assert_eq!(i.as_integer(), 42);
        
        let neg = NanBoxed::integer(-1000);
        assert!(neg.is_integer());
        assert_eq!(neg.as_integer(), -1000);
        
        // Test sign extension
        let large_neg = NanBoxed::integer(-123456789);
        assert_eq!(large_neg.as_integer(), -123456789);
    }
    
    #[test]
    fn test_string_ptr() {
        let ptr = HeapObject::new_string("hello");
        let v = NanBoxed::ptr(ptr);
        
        assert!(v.is_ptr());
        assert!(v.is_truthy());
        
        let obj = unsafe { &*v.as_ptr() };
        assert_eq!(obj.tag, ObjectTag::String);
        
        // Clean up
        unsafe { drop(Box::from_raw(ptr)); }
    }
}

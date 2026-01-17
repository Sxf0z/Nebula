use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use super::nanbox::{HeapObject, NanBoxed};

pub struct StringInterner {
    strings: HashMap<u64, *mut HeapObject>,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            strings: HashMap::with_capacity(64),
        }
    }

    #[inline]
    fn hash_str(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    pub fn intern(&mut self, s: &str) -> NanBoxed {
        let hash = Self::hash_str(s);
        
        if let Some(&ptr) = self.strings.get(&hash) {
            unsafe {
                if let super::nanbox::HeapData::String(ref existing) = (*ptr).data {
                    if existing.as_ref() == s {
                        (*ptr).incref();
                        return NanBoxed::ptr(ptr);
                    }
                }
            }
        }
        
        let ptr = HeapObject::new_string(s);
        self.strings.insert(hash, ptr);
        NanBoxed::ptr(ptr)
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interning() {
        let mut interner = StringInterner::new();
        let a = interner.intern("hello");
        let b = interner.intern("hello");
        assert_eq!(a.bits(), b.bits());
    }

    #[test]
    fn test_different_strings() {
        let mut interner = StringInterner::new();
        let a = interner.intern("hello");
        let b = interner.intern("world");
        assert_ne!(a.bits(), b.bits());
    }
}

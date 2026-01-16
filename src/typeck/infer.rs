//! Type inference engine
//!
//! Uses unification-based inference with type variables.

use std::collections::HashMap;
use super::types::Ty;

/// Type inference context
pub struct InferCtx {
    next_var: usize,
    substitutions: HashMap<usize, Ty>,
}

impl InferCtx {
    pub fn new() -> Self {
        Self {
            next_var: 0,
            substitutions: HashMap::new(),
        }
    }

    /// Create a fresh type variable
    pub fn fresh_var(&mut self) -> Ty {
        let var = self.next_var;
        self.next_var += 1;
        Ty::Var(var)
    }

    /// Unify two types, returning true if successful
    pub fn unify(&mut self, a: &Ty, b: &Ty) -> bool {
        let a = self.resolve(a);
        let b = self.resolve(b);

        match (&a, &b) {
            // Same types unify
            _ if a == b => true,

            // Type variables unify with anything
            (Ty::Var(id), _) => {
                self.substitutions.insert(*id, b);
                true
            }
            (_, Ty::Var(id)) => {
                self.substitutions.insert(*id, a);
                true
            }

            // Arrays must have same element type and size
            (Ty::Array(elem_a, size_a), Ty::Array(elem_b, size_b)) => {
                size_a == size_b && self.unify(elem_a, elem_b)
            }

            // Slices must have same element type
            (Ty::Slice(elem_a), Ty::Slice(elem_b)) => {
                self.unify(elem_a, elem_b)
            }

            // Tuples must have same arity and element types
            (Ty::Tuple(types_a), Ty::Tuple(types_b)) => {
                types_a.len() == types_b.len() &&
                types_a.iter().zip(types_b.iter()).all(|(a, b)| self.unify(a, b))
            }

            // Functions must have matching signatures
            (Ty::Function(params_a, ret_a), Ty::Function(params_b, ret_b)) => {
                params_a.len() == params_b.len() &&
                params_a.iter().zip(params_b.iter()).all(|(a, b)| self.unify(a, b)) &&
                self.unify(ret_a, ret_b)
            }

            // Error type unifies with anything (to allow recovery)
            (Ty::Error, _) | (_, Ty::Error) => true,

            // Generic types with same name and unifiable args
            (Ty::Generic(name_a, args_a), Ty::Generic(name_b, args_b)) => {
                name_a == name_b &&
                args_a.len() == args_b.len() &&
                args_a.iter().zip(args_b.iter()).all(|(a, b)| self.unify(a, b))
            }

            _ => false,
        }
    }

    /// Resolve a type to its most concrete form
    pub fn resolve(&self, ty: &Ty) -> Ty {
        match ty {
            Ty::Var(id) => {
                if let Some(resolved) = self.substitutions.get(id) {
                    self.resolve(resolved)
                } else {
                    ty.clone()
                }
            }
            Ty::Array(elem, size) => Ty::Array(Box::new(self.resolve(elem)), *size),
            Ty::Slice(elem) => Ty::Slice(Box::new(self.resolve(elem))),
            Ty::Tuple(types) => Ty::Tuple(types.iter().map(|t| self.resolve(t)).collect()),
            Ty::Function(params, ret) => Ty::Function(
                params.iter().map(|t| self.resolve(t)).collect(),
                Box::new(self.resolve(ret)),
            ),
            Ty::Generic(name, args) => Ty::Generic(
                name.clone(),
                args.iter().map(|t| self.resolve(t)).collect(),
            ),
            _ => ty.clone(),
        }
    }

    /// Apply default types to remaining type variables
    pub fn apply_defaults(&mut self, ty: &Ty) -> Ty {
        match ty {
            Ty::Var(id) => {
                if self.substitutions.contains_key(id) {
                    self.apply_defaults(&self.resolve(ty))
                } else {
                    // Default integer to i64, float to f64
                    Ty::I64
                }
            }
            Ty::Array(elem, size) => Ty::Array(Box::new(self.apply_defaults(elem)), *size),
            Ty::Slice(elem) => Ty::Slice(Box::new(self.apply_defaults(elem))),
            Ty::Tuple(types) => Ty::Tuple(types.iter().map(|t| self.apply_defaults(t)).collect()),
            Ty::Function(params, ret) => Ty::Function(
                params.iter().map(|t| self.apply_defaults(t)).collect(),
                Box::new(self.apply_defaults(ret)),
            ),
            _ => ty.clone(),
        }
    }
}

impl Default for InferCtx {
    fn default() -> Self {
        Self::new()
    }
}

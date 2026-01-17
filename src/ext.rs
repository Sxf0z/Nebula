use crate::interp::Value;
use crate::error::{SpectreError, SpectreResult, ErrorCode};
pub type ExtResult<T> = Result<T, ExtError>;
#[derive(Debug, Clone)]
pub struct ExtError {
    pub message: String,
}
impl ExtError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self { message: msg.into() }
    }
}
impl From<ExtError> for SpectreError {
    fn from(e: ExtError) -> Self {
        SpectreError::coded(ErrorCode::E080, e.message)
    }
}
pub struct ExtensionContext<'a> {
    pub fn_name: &'a str,
    pub argc: usize,
}
impl<'a> ExtensionContext<'a> {
    pub fn new(fn_name: &'a str, argc: usize) -> Self {
        Self { fn_name, argc }
    }
}
pub type NativeFn = fn(&ExtensionContext, &[Value]) -> ExtResult<Value>;
#[derive(Clone)]
pub struct ExtFunction {
    pub name: String,
    pub min_args: usize,
    pub max_args: Option<usize>,
    pub func: NativeFn,
}
impl ExtFunction {
    pub fn new(name: impl Into<String>, func: NativeFn) -> Self {
        Self {
            name: name.into(),
            min_args: 0,
            max_args: None,
            func,
        }
    }
    pub fn with_arity(name: impl Into<String>, arity: usize, func: NativeFn) -> Self {
        Self {
            name: name.into(),
            min_args: arity,
            max_args: Some(arity),
            func,
        }
    }
    pub fn validate_args(&self, argc: usize) -> ExtResult<()> {
        if argc < self.min_args {
            return Err(ExtError::new(format!(
                "{}: expected at least {} args, got {}",
                self.name, self.min_args, argc
            )));
        }
        if let Some(max) = self.max_args {
            if argc > max {
                return Err(ExtError::new(format!(
                    "{}: expected at most {} args, got {}",
                    self.name, max, argc
                )));
            }
        }
        Ok(())
    }
}
pub trait Extension: Send + Sync {
    fn name(&self) -> &str;
    fn functions(&self) -> Vec<ExtFunction>;
    fn on_load(&self) -> ExtResult<()> {
        Ok(())
    }
    fn on_unload(&self) -> ExtResult<()> {
        Ok(())
    }
}
pub struct ExtensionRegistry {
    extensions: Vec<Box<dyn Extension>>,
    functions: std::collections::HashMap<String, ExtFunction>,
}
impl ExtensionRegistry {
    pub fn new() -> Self {
        Self {
            extensions: Vec::new(),
            functions: std::collections::HashMap::new(),
        }
    }
    pub fn register(&mut self, ext: Box<dyn Extension>) -> ExtResult<()> {
        ext.on_load()?;
        for func in ext.functions() {
            self.functions.insert(func.name.clone(), func);
        }
        self.extensions.push(ext);
        Ok(())
    }
    pub fn get_function(&self, name: &str) -> Option<&ExtFunction> {
        self.functions.get(name)
    }
    pub fn call(&self, name: &str, args: &[Value]) -> SpectreResult<Value> {
        let func = self.functions.get(name)
            .ok_or_else(|| SpectreError::coded(ErrorCode::E010, name))?;
        func.validate_args(args.len())?;
        let ctx = ExtensionContext::new(name, args.len());
        (func.func)(&ctx, args).map_err(|e| e.into())
    }
}
impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn test_add(_ctx: &ExtensionContext, args: &[Value]) -> ExtResult<Value> {
        let a = args.get(0).and_then(|v| v.as_number()).unwrap_or(0.0);
        let b = args.get(1).and_then(|v| v.as_number()).unwrap_or(0.0);
        Ok(Value::Number(a + b))
    }
    #[test]
    fn test_ext_function() {
        let func = ExtFunction::with_arity("add", 2, test_add);
        assert!(func.validate_args(2).is_ok());
        assert!(func.validate_args(1).is_err());
    }
}

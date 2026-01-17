use crate::interp::{Value, NativeFn};
pub fn io_builtins() -> Vec<(&'static str, NativeFn)> {
    vec![
        ("input", NativeFn {
            name: "input".to_string(),
            arity: Some(0),
            func: |_args| {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line)
                    .map_err(|e| e.to_string())?;
                Ok(Value::String(line.trim().to_string()))
            },
        }),
        ("input_prompt", NativeFn {
            name: "input_prompt".to_string(),
            arity: Some(1),
            func: |args| {
                use std::io::{self, Write};
                print!("{}", args[0]);
                io::stdout().flush().map_err(|e| e.to_string())?;
                let mut line = String::new();
                std::io::stdin().read_line(&mut line)
                    .map_err(|e| e.to_string())?;
                Ok(Value::String(line.trim().to_string()))
            },
        }),
    ]
}

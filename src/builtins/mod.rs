use crate::interp::{Value, NativeFn};
pub fn get_builtins() -> Vec<(&'static str, NativeFn)> {
    vec![
        ("log", NativeFn {
            name: "log".to_string(),
            arity: None, 
            func: |args| {
                let output: Vec<_> = args.iter().map(|a| format!("{}", a)).collect();
                println!("{}", output.join(" "));
                Ok(Value::Nil)
            },
        }),
        ("get", NativeFn {
            name: "get".to_string(),
            arity: Some(0),
            func: |_args| {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line)
                    .map_err(|e| e.to_string())?;
                Ok(Value::String(line.trim().to_string()))
            },
        }),
        ("sqrt", NativeFn {
            name: "sqrt".to_string(),
            arity: Some(1),
            func: |args| {
                let n = args[0].as_number()
                    .ok_or("sqrt() requires numeric argument")?;
                Ok(Value::Number(n.sqrt()))
            },
        }),
        ("abs", NativeFn {
            name: "abs".to_string(),
            arity: Some(1),
            func: |args| {
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(n.abs())),
                    Value::Integer(n) => Ok(Value::Integer(n.abs())),
                    _ => Err("abs() requires numeric argument".to_string()),
                }
            },
        }),
        ("typeof", NativeFn {
            name: "typeof".to_string(),
            arity: Some(1),
            func: |args| {
                Ok(Value::String(args[0].type_name().to_string()))
            },
        }),
    ]
}

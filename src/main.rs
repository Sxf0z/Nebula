use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use std::time::Instant;

use colored::Colorize;
use specterscript::{Lexer, Parser, Interpreter, SpectreError, VM, Compiler, Value};

const BANNER: &str = r#"
▀█▄    ▀█▀         ▀██                ▀██          
 █▀█    █    ▄▄▄▄   ██ ▄▄▄  ▄▄▄ ▄▄▄    ██    ▄▄▄▄  
 █ ▀█▄  █  ▄█▄▄▄██  ██▀  ██  ██  ██    ██   ▀▀ ▄██  
 █    ███  ██       ██    █  ██  ██    ██   ▄█▀ ██  
▄█▄    ▀█   ▀█▄▄▄▀  ▀█▄▄▄▀   ▀█▄▄▀█▄  ▄██▄  ▀█▄▄▀█▀  
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (use_vm, file_path) = parse_args(&args);

    match file_path {
        None => run_repl(use_vm),
        Some(path) => run_file(&path, use_vm),
    }
}

fn parse_args(args: &[String]) -> (bool, Option<String>) {
    let mut use_vm = false;
    let mut file_path = None;

    for arg in args.iter().skip(1) {
        if arg == "--vm" {
            use_vm = true;
        } else if arg == "--help" || arg == "-h" {
            print_usage();
            process::exit(0);
        } else if arg.starts_with('-') {
            eprintln!("{} Unknown flag: {}", "[ERROR]".bold().red(), arg);
            print_usage();
            process::exit(64);
        } else {
            file_path = Some(arg.clone());
        }
    }

    (use_vm, file_path)
}

fn print_usage() {
    println!("{}", BANNER.cyan());
    println!("{}", "  Logic is Electric.".purple().italic());
    println!();
    println!("{}", "USAGE:".bold().white());
    println!("  {} {}              {}", "nebula".cyan(), "".dimmed(), "Start REPL");
    println!("  {} {}  {}", "nebula".cyan(), "<script.na>".green(), "Run script (interpreter)");
    println!("  {} {} {} {}", "nebula".cyan(), "--vm".yellow(), "<script>".green(), "Run script (fast VM)");
    println!();
    println!("{}", "OPTIONS:".bold().white());
    println!("  {}    Use bytecode VM (35x faster)", "--vm".yellow());
    println!("  {}  Show this message", "--help".yellow());
}

fn run_repl(use_vm: bool) {
    println!("{}", BANNER.cyan());
    let mode = if use_vm { "VM".green() } else { "Interpreter".blue() };
    println!("  {} {} {}", "Nebula".purple().bold(), "v1.0".dimmed(), mode);
    println!("  Type {} to quit\n", "'exit'".dimmed());

    let mut interpreter = Interpreter::new();
    let mut input = String::new();

    loop {
        print!("{} ", "λ".purple().bold());
        let _ = io::stdout().flush();

        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let line = input.trim();
        if line == "exit" || line == "quit" {
            println!("{}", "✨ Goodbye.".cyan());
            break;
        }

        if line.is_empty() {
            continue;
        }

        let start = Instant::now();
        let result = if use_vm {
            run_vm(line)
        } else {
            run_interpreter(line, &mut interpreter)
        };

        match result {
            Ok(value) => {
                if !matches!(value, Value::Nil) {
                    println!("{} {}", "=>".dimmed(), format!("{}", value).green());
                }
            }
            Err(e) => {
                println!("{} {}", "[ERROR]".bold().red(), e.message().red());
            }
        }
        
        let elapsed = start.elapsed();
        if elapsed.as_millis() > 10 {
            println!("{}", format!("  ⏱ {}ms", elapsed.as_millis()).dimmed());
        }
    }
}

fn run_file(path: &str, use_vm: bool) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Cannot read '{}': {}", "[FILE ERROR]".bold().red(), path.yellow(), e);
            process::exit(66);
        }
    };

    let start = Instant::now();
    
    let result = if use_vm {
        run_vm(&source)
    } else {
        let mut interpreter = Interpreter::new();
        run_interpreter(&source, &mut interpreter)
    };

    let elapsed = start.elapsed();

    match result {
        Ok(_) => {
            println!("{}", format!("✨ Executed in {:.3}s", elapsed.as_secs_f64()).cyan());
        }
        Err(e) => {
            report_error(&source, &e);
            process::exit(70);
        }
    }
}

fn run_interpreter(source: &str, interpreter: &mut Interpreter) -> Result<Value, SpectreError> {
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    for token in &tokens {
        if let specterscript::TokenKind::Error(msg) = &token.kind {
            return Err(SpectreError::Lexer {
                message: msg.clone(),
                span: token.span,
            });
        }
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    interpreter.interpret(&program)
}

fn run_vm(source: &str) -> Result<Value, SpectreError> {
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();

    for token in &tokens {
        if let specterscript::TokenKind::Error(msg) = &token.kind {
            return Err(SpectreError::Lexer {
                message: msg.clone(),
                span: token.span,
            });
        }
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&program)?;
    let global_names = compiler.global_names();
    let functions = compiler.functions();

    let mut vm = VM::new();
    let result = vm.run_with_functions(&chunk, global_names, functions)?;
    
    Ok(nanbox_to_value(result))
}

fn nanbox_to_value(nb: specterscript::vm::NanBoxed) -> Value {
    if nb.is_nil() {
        Value::Nil
    } else if nb.is_bool() {
        Value::Bool(nb.as_bool())
    } else if nb.is_number() {
        Value::Number(nb.as_number())
    } else if nb.is_integer() {
        Value::Integer(nb.as_integer())
    } else if nb.is_ptr() {
        let obj = unsafe { &*nb.as_ptr() };
        match &obj.data {
            specterscript::vm::HeapData::String(s) => Value::String(s.to_string()),
            specterscript::vm::HeapData::List(items) => {
                Value::List(items.iter().map(|v| nanbox_to_value(*v)).collect())
            }
            specterscript::vm::HeapData::Map(map) => {
                Value::Map(map.iter().map(|(k, v)| (k.to_string(), nanbox_to_value(*v))).collect())
            }
            specterscript::vm::HeapData::Function(f) => {
                Value::String(format!("<fn {}>", f.name))
            }
        }
    } else {
        Value::Nil
    }
}

fn report_error(source: &str, error: &SpectreError) {
    eprintln!("{}", "[COSMIC FRACTURE]".bold().red());
    eprintln!("{}", error.message().red());
    
    if let Some(span) = error.span() {
        let lines: Vec<_> = source.lines().collect();
        if span.line > 0 && span.line <= lines.len() {
            let line_content = lines[span.line - 1];
            eprintln!("  {} line {}", "-->".cyan(), span.line);
            eprintln!("   {}", "|".cyan());
            eprintln!("{:3} {} {}", span.line, "|".cyan(), line_content);
            eprintln!("   {} {}^", "|".cyan(), " ".repeat(span.column.saturating_sub(1)));
        }
    }
}

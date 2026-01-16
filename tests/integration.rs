//! Integration tests for SpecterScript VM
//! 
//! Tests verify that programs compile and run without crashing.
//! Return value tests use variable reads which do return values.

use specterscript::{Lexer, Parser, Compiler, VM};

/// Run code through VM - returns Ok if no crash/error
fn run(code: &str) -> Result<(), String> {
    let tokens: Vec<_> = Lexer::new(code).collect();
    let program = Parser::new(tokens).parse_program().map_err(|e| e.message())?;
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&program).map_err(|e| e.message())?;
    let mut vm = VM::new();
    vm.run_with_functions(&chunk, compiler.global_names(), compiler.functions()).map_err(|e| e.message())?;
    Ok(())
}

/// Run code expecting an error
fn expect_err(code: &str) -> bool {
    run(code).is_err()
}

// === Compile & Run Tests (no crashes) ===

#[test]
fn test_arithmetic_compiles() {
    run("fb x = 1 + 2").unwrap();
    run("fb x = 10 - 3").unwrap();
    run("fb x = 6 * 7").unwrap();
    run("fb x = 10 / 4").unwrap();
    run("fb x = 10 % 3").unwrap();
}

#[test]
fn test_variables_compile() {
    run("fb x = 42").unwrap();
    run("fb x = 1\nx = 2").unwrap();
    run("fb a = 1\nfb b = 2\nfb c = a + b").unwrap();
}

#[test]
fn test_control_flow_compiles() {
    run("fb x = 0\nif true do\n  x = 1\nend").unwrap();
    run("fb x = 0\nif false do\n  x = 1\nelse\n  x = 2\nend").unwrap();
    run("fb x = 0\nwhile x < 5 do\n  x = x + 1\nend").unwrap();
    run("fb sum = 0\nfor i = 1, 5 do\n  sum = sum + i\nend").unwrap();
}

#[test]
fn test_comparisons_compile() {
    run("fb x = 1 == 1").unwrap();
    run("fb x = 1 != 2").unwrap();
    run("fb x = 1 < 2").unwrap();
    run("fb x = 2 > 1").unwrap();
    run("fb x = 1 <= 1").unwrap();
    run("fb x = 2 >= 2").unwrap();
}

#[test]
fn test_builtins_compile() {
    run("fb x = sqrt(16)").unwrap();
    run("fb x = abs(-5)").unwrap();
    run("log(\"hello\")").unwrap();
}

#[test]
fn test_nested_loops() {
    run("fb sum = 0\nfor i = 1, 3 do\n  for j = 1, 3 do\n    sum = sum + 1\n  end\nend").unwrap();
}

#[test]
fn test_complex_expressions() {
    run("fb x = (1 + 2) * 3 - 4").unwrap();
    run("fb x = 10 / 2 + 3 * 4").unwrap();
}

// === Error Handling Tests ===

#[test]
fn test_divide_by_zero_error() {
    assert!(expect_err("fb x = 1 / 0"));
}

#[test]
fn test_iteration_limit() {
    // This should hit iteration limit (1M iterations)
    // Use 1 == 1 instead of true for guaranteed parse
    assert!(expect_err("fb x = 0\nwhile 1 == 1 do\n  x = x + 1\nend"));
}

// === Regression Tests (from parity_test.sp) ===

#[test]
fn test_parity_arithmetic() {
    run("fb a = 10 + 20 * 2").unwrap();
}

#[test]
fn test_parity_comparisons() {
    run("fb gt = 5 > 3\nfb lt = 5 < 3\nfb eq = 5 == 5").unwrap();
}

#[test]
fn test_parity_nested_loops() {
    run("fb sum = 0\nfor i = 1, 3 do\n  for j = 1, 3 do\n    sum = sum + 1\n  end\nend").unwrap();
}

#[test]
fn test_parity_if_elif_else() {
    run("fb x = 2\nif x == 1 do\n  log(\"one\")\nelif x == 2 do\n  log(\"two\")\nelse\n  log(\"other\")\nend").unwrap();
}

#[test]
fn test_parity_while() {
    run("fb count = 0\nwhile count < 5 do\n  count = count + 1\nend").unwrap();
}

#[test]
fn test_parity_negation() {
    run("fb neg = -10\nfb abs_neg = abs(neg)").unwrap();
}

#[test]
fn test_parity_strings() {
    run("fb msg = \"hello\"\nfb length = len(msg)").unwrap();
}

#[test]
fn test_parity_booleans() {
    run("fb t = true\nfb f = false").unwrap();
}

#[test]
fn test_parity_math() {
    run("fb sq = sqrt(16)\nfb pw = pow(2, 8)").unwrap();
}

// === Memory Tracking Tests ===

#[test]
fn test_heap_stats_available() {
    // Just verify the tracking functions compile and run
    // Note: Due to test parallelism, we can't guarantee zero counts
    let (alloc, dealloc) = specterscript::vm::heap_stats();
    // Just verify these return reasonable values (not panicking)
    assert!(alloc >= dealloc, "alloc {} should be >= dealloc {}", alloc, dealloc);
}

#[test]
fn test_string_allocation_tracked() {
    specterscript::vm::reset_stats();
    
    // Run code that creates a string 
    run("fb msg = \"test\"").unwrap();
    
    let (alloc, _) = specterscript::vm::heap_stats();
    // At least one string should be allocated (the string constant)
    assert!(alloc >= 1, "Expected at least 1 allocation, got {}", alloc);
}

// === Function Tests ===

#[test]
fn test_simple_function() {
    run("fn double(x) = x * 2\nfb r = double(5)").unwrap();
}

#[test]
fn test_multi_param_function() {
    run("fn add(a, b) = a + b\nfb r = add(3, 4)").unwrap();
}

#[test]
fn test_nested_function_calls() {
    run("fn square(x) = x * x\nfb r = square(square(2))").unwrap();
}

#[test]
fn test_block_function() {
    run("fn greet(name) do\n  log(name)\nend\ngreet(\"World\")").unwrap();
}

#[test]
fn test_zero_param_function() {
    run("fn zero() = 0\nfb r = zero()").unwrap();
}

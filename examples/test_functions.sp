# Test user-defined functions

# Simple expression function
fn double(x) = x * 2
log("double(5) =", double(5))

# Two-parameter function
fn add(a, b) = a + b
log("add(3, 4) =", add(3, 4))

# Nested calls
fn square(x) = x * x
log("square(square(2)) =", square(square(2)))

# Block function (to test block body)
fn greet(name) do
    log("Hello,", name)
end
greet("World")

# Function returning 0
fn zero() = 0
log("zero() =", zero())

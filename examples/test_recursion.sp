# Test recursion limit
fn recurse(n) do
    log("Depth:", n)
    recurse(n + 1)
end

recurse(1)

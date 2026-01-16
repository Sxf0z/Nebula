# Simpler nested test - trace values
fb sum = 0
for i = 1, 2 do
    log("i=", i)
    for j = 1, 2 do
        log("  j=", j)
        sum = sum + 1
    end
end
log("total:", sum)

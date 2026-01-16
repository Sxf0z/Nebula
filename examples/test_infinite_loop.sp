# Test infinite loop detection
fb x = 0
while yes do
    x += 1
end
log("Should not reach here")

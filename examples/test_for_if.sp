# Test if condition with local var comparison
for i = 1, 3 do
    fb result = i == 2
    log("i=", i, "result=", result)
    if result do
        log("MATCH")
    end
end

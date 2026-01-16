# Structs example in SpecterScript

# Create a map to represent a point
fb point = map("x": 10, "y": 20)
log("Point:", point)
log("x:", point.x)
log("y:", point.y)

# Function that works with points
fn distance(p) do
    fb x = p.x
    fb y = p.y
    -> sqrt(x * x + y * y)
end

log("Distance from origin:", distance(point))

# Create another point
fb point2 = map("x": 3, "y": 4)
log("Point2 distance:", distance(point2))

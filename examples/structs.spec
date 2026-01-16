# Structs and methods in SpecterScript

struct Point:
    x: f64
    y: f64

impl Point:
    fn new(x: f64, y: f64) -> Point:
        return Point { x: x, y: y }

    fn distance(self) -> f64:
        return sqrt(self.x * self.x + self.y * self.y)

    fn add(self, other: Point) -> Point:
        return Point { x: self.x + other.x, y: self.y + other.y }

fn main():
    p1 = Point::new(3.0, 4.0)
    p2 = Point::new(1.0, 2.0)
    
    print("Point 1:")
    print(p1.x)
    print(p1.y)
    
    dist = p1.distance()
    print("Distance from origin:")
    print(dist)
    
    p3 = p1.add(p2)
    print("Sum of points:")
    print(p3.x)
    print(p3.y)

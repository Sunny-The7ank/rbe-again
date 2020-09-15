/*
####### NOTES #######
There are 3 types of structures ("structs") that can be created using the struct keyword
    Tuple structs, which are, basically, named tuples
    The classic C structs
    Unit structs, which are field-less, are useful for generics

*/

#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    p1: Point,
    p2: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let &Rectangle { 
        p1: Point { x: x1, y: y1 }, 
        p2: Point { x: x2, y: y2 }
    } = rect;

    return (x2 - x1).abs() * (y2 - y1).abs();
}

fn square(origin: Point, dimension: f32) -> Rectangle {
    let Point {
        x, y
    } = origin;

    Rectangle{ p1: origin, p2: Point { x: x + dimension, y: y + dimension}}
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: left_edge, y: top_edge },
        p2: bottom_right,
    };
    let area = rect_area(&rectangle);
    println!("Area of rectangle: {:?} is {}", rectangle, area);

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let new_square = square(Point {x: 0.5, y: 0.5}, 1.3);

    println!("Square: {:?}", new_square);
}

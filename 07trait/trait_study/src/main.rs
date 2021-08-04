use std::fmt::{Display, Formatter, Result };

trait Geometry {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Rectangle: ({}, {})", self.width, self.height)
           }
}

impl Geometry for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        (self.width + self.height) * 2.0
    }
}

struct Circle {
    radius: f32,
}

impl Geometry for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }

    fn perimeter(&self) -> f32 {
        3.14 * 2.0 * self.radius
    }
}

fn print(geometry: impl Geometry + Display) {
    println!("{}, area: {}, perimeter: {}",
        geometry, geometry.area(), geometry.perimeter());
}

fn area_add(geo1: impl Geometry, geo2: impl Geometry) {
    println!("rect.area: {}, circle.area: {}, total area: {}",
        geo1.area(), geo2.area(), geo1.area() + geo2.area())
}


fn main() {
    let rect = Rectangle { width: 8.8, height: 2.2 };
    println!("rect.area: {}, rect.perimeter: {} ", rect.area(), rect.perimeter());

    let circle = Circle {radius: 2.0 };
    println!("circle.area: {}, circle.perimeter: {}", circle.area(), circle.perimeter());

    let rect2 = Rectangle {width: 2.0, height: 3.0};
//    print(rect);

    area_add(rect, circle);
}


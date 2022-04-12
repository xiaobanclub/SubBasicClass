use std::ops::Div;
use std::ops::Mul;

trait Area<T> {
    fn area(&self) -> T;
}

#[derive(Debug)]
struct Square<T> {
    width: T,
}

impl<T: Copy + Mul<Output = T>> Area<T> for Square<T> {
    fn area(&self) -> T {
        self.width * self.width
    }
}

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Area<T> for Rectangle<T, U>
where
    T: Mul<U, Output = T> + Copy,
    U: Mul<T, Output = T> + Copy,
{
    fn area(&self) -> T {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Triangle<T> {
    base: T,
    height: T,
}

impl<T: Copy + Mul<Output = T> + Div<f64, Output = T>> Area<T> for Triangle<T> {
    fn area(&self) -> T {
        (self.base * self.height) / 2.0
    }
}

#[derive(Debug)]
struct Circle<T> {
    radius: T,
}

impl<T: Copy + Mul<Output = T> + Mul<f64, Output = T>> Area<T> for Circle<T> {
    fn area(&self) -> T {
        self.radius * self.radius * 3.14
    }
}

fn get_area<T>(graph: &impl Area<T>) -> T {
    return graph.area();
}

fn main() {
    let a = Square { width: 5 };
    let area = get_area(&a);
    println!("{}", area); // 25

    let a = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    let area = get_area(&a);
    println!("{}", area); // 12

    let a = Triangle {
        base: 3.0,
        height: 4.0,
    };
    let area = get_area(&a);
    println!("{}", area); // 6

    let a = Circle { radius: 5.0 };
    let area = get_area(&a);
    println!("{}", area); // 78.5
}

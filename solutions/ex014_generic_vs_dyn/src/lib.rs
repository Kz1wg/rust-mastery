pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

/// ```compile_fail
/// use ex014_generic_vs_dyn::{Circle, Square};
///
/// let shapes = vec![Circle { radius: 1.0 }, Square { side: 2.0 }]; // 型が違う
/// ```
pub fn total_area_generic<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(Shape::area).sum()
}

pub fn total_area_dyn(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

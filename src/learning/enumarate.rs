// enum
pub enum Shape{
    Rectangle(f64, f64),
    Circle(f64)
}

pub fn calc_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(w, h) => w * h,
        Shape::Circle(r) => 3.14 * r * r,
    }
}
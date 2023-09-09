

#[derive(Debug)]
pub struct Vector2 {
    x: f64,
    y: f64
}

impl Vector2 {

    pub fn new(x: f64, y: f64) -> Self {//Vector2 {
        Vector2 { x, y }
    }

    pub fn add(first_vector: Vector2, second_vector: Vector2) -> Vector2 {
        Vector2::new(first_vector.x + second_vector.x, first_vector.y + second_vector.y)
    }
}







fn main() {

    let first_vector = Vector2::new(1.0, 2.0);
    let second_vector = Vector2::new(3.0, 4.0);
    let summed_vector = Vector2::add(first_vector, second_vector);

    println!("{:?}", summed_vector);

}

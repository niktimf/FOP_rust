
/// Макрос, который автоматически реализует trait Debug для нашего типа, чтобы мы смогли вывести
/// значение с помощью println!
#[derive(Debug)]
pub struct Vector2 {
    x: f64,
    y: f64
}

impl Vector2 {

    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn add(first_vector: Vector2, second_vector: Vector2) -> Vector2 {
        Vector2::new(first_vector.x + second_vector.x, first_vector.y + second_vector.y)
    }
}

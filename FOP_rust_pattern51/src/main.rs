mod vector2;


use crate::vector2::Vector2;



fn main() {
    let first_vector = Vector2::new(1.0, 2.0);
    let second_vector = Vector2::new(3.0, 4.0);
    let summed_vector = Vector2::add(first_vector, second_vector);

    /// :? указывает на то, что значение должно быть отформатировано с использованием реализации трейта
    /// Debug, то есть Rust выводит значение используя реализацию Debug
    println!("{:?}", summed_vector);
}

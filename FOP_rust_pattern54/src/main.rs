
/// Несмотря на то, что Rust является системным языком,
/// он предоставляет ряд инструментов для написания декларативного кода.
/// Причем в идиоматичном rust вы почти всегда пишете код в декларативном стиле.
/// https://doc.rust-lang.org/book/ch13-02-iterators.html
/// https://rust-unofficial.github.io/patterns/functional/paradigms.html


/// Рассмотрим простую задачу.
/// Допустим у нас есть число и есть диапазон(от большего числа к меньшему), нужно определить принадлежность числа к диапазону.

/// Напишем функцию в императивном стиле.
fn is_number_in_range_imperative(start: u32, end: u32, number: u32) -> bool {
    number >= start && number <= end
}

/// Напишем функцию в декларативном стиле
fn is_number_in_range_declarative(start: u32, end: u32, number: u32) -> bool {
    (start..end).contains(&number)
}

fn main() {
    is_number_in_range_imperative(1, 10, 5);
    is_number_in_range_declarative(1, 10, 5);
}

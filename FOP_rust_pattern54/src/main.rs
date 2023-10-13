


/// Несмотря на то, что Rust является систеным языком,
/// он предоставляет ряд инструментов для написания декларативного кода.
/// Причем в идиоматичном rust вы почти всегда пишете код в декларативном стиле.

/// Рассмторим простую задачу.
/// Допустим у нас есть число и есть диапазон(от большего числа к меньшему), нужно определить принадлжежность числа к диапазону.


/// Напишем функцию в императивном стиле.

fn is_number_in_range_imperative(number: u32, start: u32, end: u32) {
    if start > end {
        println!("Неверный диапазон");
        return;
    } else if number >= start && number <= end {
        println!("Число {} принадлежит диапазону {} - {}", number, start, end);
    } else {
        println!("Число {} не принадлежит диапазону {} - {}", number, start, end);
    }
}


/// Напишем функцию в декларативном стиле

fn is_number_in_range_declarative(number: u32, start: u32, end: u32) {
    match start > end {
        true => return println!("Неверный диапазон"),
        false => {
            match (start..end).contains(&number) {
                true => println!("Число {} принадлежит диапазону {} - {}", number, start, end),
                false => println!("Число {} не принадлежит диапазону {} - {}", number, start, end)
            }
        }
    }
}



fn main() {
    is_number_in_range_imperative(5, 1, 10);
    is_number_in_range_declarative(5, 1, 10);
}








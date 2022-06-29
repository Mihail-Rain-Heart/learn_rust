// Простая структура
struct Simple {
    field: i32,
    field2: bool,
    field3: u32,
}

// Без именования
struct Color(i32, i32, i32);

// Единичная стуктура
struct Equal;

fn main() {
    let simple = Simple {
        field: 1,
        field2: true,
        field3: 3,
    };

    let simple = Simple {
        field3: 123,
        ..simple
    };

    let color = Color(1, 2, 3);
    let Color { 2: c, .. } = color;
    let Simple {
        field: a,
        field2: b,
        field3: simple1,
    } = simple;

    println!("{}", simple.field3);
    println!("{} {} a: {}, b: {}, field: {}", color.2, c, a, b, simple1);

    let Simple { field2: a, .. } = simple;
    println!("{}", a)
}

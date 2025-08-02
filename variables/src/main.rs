fn main() {
    // const MAX_POINTS: u32 = 100_000;

    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("Значение x равно {}", x);
    
    let spaces = "     ";
    let spaces = spaces.len();
    println!("Значение spaces равно {}", spaces);
    
    // Типы данных
    // типы с плавающей точкой
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    
    // числовые операции
    // сложение
    let sum = 5 + 10;

    // вычитание
    let difference = 95.5 - 4.3;

    // умножение
    let product = 4 * 30;

    // деление
    let quotient = 56.7 / 32.2;

    // остаток
    let remainder = 43 % 5;

    // булев тип
    let t = true;
    let f: bool = false; // с явной аннотацией

    // символьный тип
    let c = 'z';
    let z = '@';
    let heart_eyed_cat = '';

    // составные типы
    // кортежный тип
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

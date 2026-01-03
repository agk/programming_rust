fn main() {
    println!("Hello, World!");
    another_function();

    // Инструкции и выражения в телах функций
    let _xd = 5;
    let yd = {
        let xd = 3;
        xd + 1
    };
    println!("Значение yd равно {}", yd);
    
    // Функции с возвращаемыми значениями
    let xf = five();
    println!("Значение xf равно {}", xf);

    let x = plus_one(5);
    println!("Значение x равно {}", x);
}

fn another_function() {
    println!("Еще одна функция.");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
